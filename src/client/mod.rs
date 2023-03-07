use std::{collections::HashSet, net::SocketAddr, sync::Arc, time::Duration};

use binbuf::prelude::*;
use rand;
use tokio::{net::UdpSocket, time::Instant};

use crate::{
    constants::udp::MIN_MESSAGE_SIZE,
    types::{
        dns::{Header, Message, Query, Question, ToQuery},
        udp::Session,
    },
    utils::{timeout, TimeoutResult},
};

mod error;

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
    pub async fn query<Q: ToQuery>(
        &self,
        query: Q,
        addr: SocketAddr,
    ) -> ClientResult<(Message, usize)> {
        let active_ids = self.active_ids.clone();
        let query = query.to_query();

        // NOTE (Techassi): Can we avoid cloning here?
        let write_timeout = self.write_timeout.clone();
        let read_timeout = self.read_timeout.clone();
        let buffer_size = self.buffer_size.clone();

        let session = Session {
            socket: self.socket.clone(),
            addr,
        };

        // TODO (Techassi): Pass the timeouts defined in the client
        let result = tokio::spawn(async move {
            do_query(
                query,
                session,
                active_ids,
                write_timeout,
                read_timeout,
                buffer_size,
            )
            .await
        });

        // TODO (Techassi): Remove transaction ID from active_ids when done
        match result.await {
            Ok(res) => match res {
                Ok(msg) => Ok(msg),
                Err(err) => Err(err),
            },
            Err(err) => Err(ClientError::RuntimeError(err)),
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
    pub async fn query_duration<Q: ToQuery>(
        &self,
        query: Q,
        addr: SocketAddr,
    ) -> ClientResult<(Message, usize, Duration)> {
        let now = Instant::now();
        let (message, len) = self.query(query, addr).await?;
        Ok((message, len, now.elapsed()))
    }
}

async fn do_query(
    query: Query,
    session: Session,
    active_ids: Arc<HashSet<u16>>,
    write_timeout: u64,
    read_timeout: u64,
    buffer_size: usize,
) -> ClientResult<(Message, usize)> {
    let id = get_free_transaction_id(active_ids);

    let mut message = Message::new_with_header(Header::new(id));
    message.add_question(Question::from(query));

    let mut buf = WriteBuffer::new();
    message.write::<BigEndian>(&mut buf)?;

    let write_timeout = Duration::from_secs(write_timeout);
    let read_timeout = Duration::from_secs(read_timeout);

    // Send DNS query to the remote DNS server
    match timeout(
        write_timeout,
        session.socket.send_to(buf.bytes(), session.addr),
    )
    .await
    {
        TimeoutResult::Timeout => return Err(ClientError::WriteTimeout(write_timeout)),
        TimeoutResult::Error(err) => return Err(ClientError::IO(err)),
        TimeoutResult::Ok(_) => {}
    }

    // Wait for the DNS response
    match timeout(read_timeout, wait_for_query_response(session, buffer_size)).await {
        TimeoutResult::Timeout => Err(ClientError::ReadTimeout(read_timeout)),
        TimeoutResult::Error(err) => Err(err),
        TimeoutResult::Ok(msg) => Ok(msg),
    }
}

async fn wait_for_query_response(
    session: Session,
    buffer_size: usize,
) -> ClientResult<(Message, usize)> {
    loop {
        session.socket.readable().await?;

        let mut buf = vec![0u8; buffer_size];
        let (len, addr) = match session.socket.recv_from(&mut buf).await {
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
        if addr != session.addr {
            continue;
        }

        return handle_query_response(&buf[..len]).await.map(|r| (r, len));
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

pub struct ClientBuilder {
    write_timeout: u64,
    buffer_size: usize,
    bind_timeout: u64,
    read_timeout: u64,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            buffer_size: MIN_MESSAGE_SIZE,
            write_timeout: 2,
            bind_timeout: 2,
            read_timeout: 2,
        }
    }
}

impl ClientBuilder {
    pub async fn build(&self) -> Result<Client, ClientError> {
        let bind_timeout = Duration::from_secs(self.bind_timeout);

        let socket = match timeout(bind_timeout, UdpSocket::bind("0.0.0.0:0")).await {
            TimeoutResult::Timeout => return Err(ClientError::WriteTimeout(bind_timeout)),
            TimeoutResult::Error(err) => return Err(ClientError::IO(err)),
            TimeoutResult::Ok(socket) => socket,
        };

        Ok(Client {
            active_ids: Arc::new(HashSet::new()),
            write_timeout: self.write_timeout,
            read_timeout: self.read_timeout,
            buffer_size: self.buffer_size,
            socket: Arc::new(socket),
        })
    }

    /// Customize the socket bind timeout.
    pub fn with_bind_timeout(&mut self, bind_timeout: u64) -> &Self {
        self.bind_timeout = bind_timeout;
        self
    }

    /// Customize the socket read timeout.
    pub fn with_read_timeout(&mut self, read_timeout: u64) -> &Self {
        self.read_timeout = read_timeout;
        self
    }

    /// Customize the socket write timeout.
    pub fn with_write_timeout(&mut self, write_timeout: u64) -> &Self {
        self.write_timeout = write_timeout;
        self
    }

    /// Customize the buffer size for receiving DNS responses. An minimum size
    /// if 512 octets is enforced. Providing a buffer size below this size
    /// will result in a buffer size of 512 octets.
    pub fn with_buffer_size(&mut self, buffer_size: usize) -> &Self {
        self.buffer_size = if buffer_size < MIN_MESSAGE_SIZE {
            MIN_MESSAGE_SIZE
        } else {
            buffer_size
        };

        self
    }
}
