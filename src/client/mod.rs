use std::{collections::HashSet, net::SocketAddr, sync::Arc, time::Duration};

use binbuf::prelude::*;
use rand;
use tokio::{net::UdpSocket, task::JoinSet, time::Instant};

use crate::{
    types::dns::{Header, Message, Query, Question, ToQuery},
    utils::{timeout, TimeoutResult},
};

mod builder;
mod error;

pub use builder::*;
pub use error::*;

pub type ClientResult<T> = Result<T, ClientError>;

pub struct Client {
    buffer_size: usize,
    write_timeout: u64,
    read_timeout: u64,

    active_ids: Arc<HashSet<u16>>,
    socket: Arc<UdpSocket>,
}

// TODO (Techassi): Implement a connection struct which finally binds to a socket.
// This connection then provides methods like query().

impl Client {
    /// Tries to create a new DNS [`Client`] with default settings. This client
    /// tries to bind to a random unused port for communication with other DNS
    /// servers. This is achieved by binding to port `0`, which instructs the
    /// OS to bind to an unused port.
    pub async fn new() -> ClientResult<Client> {
        Client::builder().build().await
    }

    /// Returns a [`ClientBuilder`] to declaratively build a [`Client`].
    ///
    /// ### Example
    ///
    /// This creates a default client and binds the socket to a random port.
    ///
    /// ```ignore
    /// use portal::client::Client;
    ///
    /// let c = Client::builder().build().await;
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    // TODO (Techassi): Add query_multi and query_multi_duration methods

    /// Sends a query to `addr` asking for `name`, `ty` and `class`.
    ///
    /// ### Example
    ///
    /// ```ignore
    /// use std::net::SocketAddr;
    /// use portal::{client::Client, types::{dns::Name, rr::{Class, Type}}}
    ///
    /// let client = Client::new().await.unwrap();
    /// let addr: SocketAddr = "1.1.1.1:53".parse();
    ///
    /// client.query((Name::try_from("example.com"), Type::A, Class:IN), addr);
    /// ```
    pub async fn query<Q>(
        &self,
        query: Q,
        target_addrs: Vec<SocketAddr>,
    ) -> ClientResult<(Message, usize, SocketAddr)>
    where
        Q: ToQuery,
    {
        // TODO (Techassi): Most of this stuff will be replaced by the DNS multiplexer
        let mut set = JoinSet::new();
        let query = query.to_query();

        for target in target_addrs {
            let active_ids = self.active_ids.clone();
            let query = query.clone();

            // NOTE (Techassi): Can we avoid cloning here?
            let write_timeout = self.write_timeout.clone();
            let read_timeout = self.read_timeout.clone();
            let buffer_size = self.buffer_size.clone();
            let socket = self.socket.clone();

            set.spawn(async move {
                do_query(
                    query,
                    socket,
                    target,
                    active_ids,
                    write_timeout,
                    read_timeout,
                    buffer_size,
                )
                .await
            });
        }

        match set.join_next().await {
            Some(result) => {
                drop(set);

                return match result {
                    Ok(res) => match res {
                        Ok(msg) => Ok(msg),
                        Err(err) => Err(err),
                    },
                    Err(err) => Err(ClientError::RuntimeError(err)),
                };
            }
            None => todo!(),
        }
    }

    /// Sends a query to `addr` asking for `name`, `ty` and `class`. In addition
    /// to `query`, we also track how long the query took and return the
    /// duration.
    ///
    /// ### Example
    ///
    /// ```ignore
    /// use std::net::SocketAddr;
    /// use portal::{client::Client, types::{dns::Name, rr::{Class, Type}}}
    ///
    /// let client = Client::new().await.unwrap();
    /// let addr: SocketAddr = "1.1.1.1:53".parse();
    ///
    /// client.query_duration((Name::try_from("example.com"), Type::A, Class:IN), addr);
    /// ```
    pub async fn query_duration<Q>(
        &self,
        query: Q,
        target_addrs: Vec<SocketAddr>,
    ) -> ClientResult<(Message, usize, Duration, SocketAddr)>
    where
        Q: ToQuery,
    {
        let now = Instant::now();
        let (message, len, target) = self.query(query, target_addrs).await?;
        Ok((message, len, now.elapsed(), target))
    }
}

async fn do_query(
    query: Query,
    socket: Arc<UdpSocket>,
    target: SocketAddr,
    active_ids: Arc<HashSet<u16>>,
    write_timeout: u64,
    read_timeout: u64,
    buffer_size: usize,
) -> ClientResult<(Message, usize, SocketAddr)> {
    let id = get_free_transaction_id(active_ids);

    let mut message = Message::new_with_header(Header::new(id));
    message.add_question(Question::from(query));

    let mut buf = WriteBuffer::new();
    message.write::<BigEndian>(&mut buf)?;

    let write_timeout = Duration::from_secs(write_timeout);
    let read_timeout = Duration::from_secs(read_timeout);

    // Send DNS query to the remote DNS server
    match timeout(write_timeout, socket.send_to(buf.bytes(), target)).await {
        TimeoutResult::Timeout => return Err(ClientError::WriteTimeout(write_timeout)),
        TimeoutResult::Error(err) => return Err(ClientError::IO(err)),
        TimeoutResult::Ok(_) => {}
    }

    // Wait for the DNS response
    match timeout(
        read_timeout,
        wait_for_query_response(socket, target, buffer_size),
    )
    .await
    {
        TimeoutResult::Timeout => Err(ClientError::ReadTimeout(read_timeout)),
        TimeoutResult::Error(err) => Err(err),
        TimeoutResult::Ok(msg) => Ok(msg),
    }
}

async fn wait_for_query_response(
    socket: Arc<UdpSocket>,
    target: SocketAddr,
    buffer_size: usize,
) -> ClientResult<(Message, usize, SocketAddr)> {
    loop {
        socket.readable().await?;

        let mut buf = vec![0u8; buffer_size];
        let (len, addr) = match socket.recv_from(&mut buf).await {
            Ok(result) => result,
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                // Continue when the socket.readable() call produced a
                // false positive
                continue;
            }
            Err(err) => {
                // TODO (Techassi): Log this
                println!("{err}");
                continue;
            }
        };

        // Skip packets which weren't received from the correct remote addr

        // FIXME: This is currently broken, as we query multiple targets at once. It is very likely we receive a valid
        // answer from a different target then currently assumed in this async task. To handle these situations, we
        // need to implement a DNS multiplexer. For now we don't ignore the message.
        if addr != target {
            // continue;
        }

        return handle_query_response(&buf[..len])
            .await
            .map(|r| (r, len, target));
    }
}

async fn handle_query_response(buf: &[u8]) -> ClientResult<Message> {
    let mut buf = ReadBuffer::new(buf);

    let header = Header::read::<BigEndian>(&mut buf)?;
    // Check transaction ID to match. Implement fn accept::accept_as_client
    let message = Message::read::<BigEndian>(&mut buf, header)?;

    Ok(message)
}

fn get_free_transaction_id(active_ids: Arc<HashSet<u16>>) -> u16 {
    let mut id = rand::random::<u16>();

    // Reroll until we get a free transaction ID
    while active_ids.contains(&id) {
        id = rand::random::<u16>();
    }

    id
}
