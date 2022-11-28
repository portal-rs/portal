use std::{
    net::{IpAddr, SocketAddr},
    time,
};

use rand;
use tokio::net::UdpSocket;

use crate::{
    constants,
    packing::{PackBuffer, Packable, UnpackBuffer, Unpackable},
    types::{
        dns::{Header, Message, Name, Question},
        rr::{Class, Type},
    },
    utils::{timeout, Network, TimeoutResult},
};

mod error;

pub use error::*;

pub type ClientResult<T> = Result<T, ClientError>;

pub struct Client {
    network: Network,
    bind_timeout: time::Duration,
    read_timeout: time::Duration,
    write_timeout: time::Duration,
    header_id: u16,
    socket: UdpSocket,
}

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
    /// ```
    /// use portal::client::Client;
    ///
    /// let c = Client::builder().build().await;
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Sends a query to `addr` asking for `name`, `class` and `ty`.
    pub async fn query(
        &self,
        name: Name,
        class: Class,
        ty: Type,
        addr: IpAddr,
    ) -> ClientResult<()> {
        let start = time::Instant::now();

        let id = rand::random::<u16>();
        let header = Header::new(id);
        let mut message = Message::new_with_header(header);

        message.add_question(Question { name, class, ty });

        let mut buf = PackBuffer::new();
        if let Err(err) = message.pack(&mut buf) {
            return Err(ClientError::WriteToBuf(err));
        }

        // Send DNS query to the remote DNS server
        match timeout(
            self.write_timeout,
            self.socket.send_to(buf.bytes(), (addr, 53)),
        )
        .await
        {
            TimeoutResult::Timeout => return Err(ClientError::WriteTimeout(self.write_timeout)),
            TimeoutResult::Error(err) => return Err(ClientError::IO(err)),
            TimeoutResult::Ok(_) => {}
        }

        // Wait for the DNS response
        match timeout(self.read_timeout, self.wait_for_query_response(addr)).await {
            TimeoutResult::Timeout => Err(ClientError::ReadTimeout(self.read_timeout)),
            TimeoutResult::Error(err) => Err(err),
            TimeoutResult::Ok(_) => {
                println!("{:?}", start.elapsed());
                Ok(())
            }
        }
    }

    async fn wait_for_query_response(&self, remote_addr: IpAddr) -> ClientResult<()> {
        loop {
            self.socket.readable().await?;

            let mut buf = [0u8; constants::udp::MIN_MESSAGE_SIZE];
            let (len, addr) = match self.socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    // Continue when the socket.readable() call procduced a
                    // false positive
                    continue;
                }
                Err(err) => {
                    // TODO (Techassi): Log this
                    println!("{}", err);
                    continue;
                }
            };

            // Skip packets which weren't recived from the correct remote addr
            if addr != SocketAddr::new(remote_addr, 53) {
                continue;
            }

            match self.handle_query_response(&buf[..len]).await {
                Ok(_) => break,
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    async fn handle_query_response(&self, buf: &[u8]) -> ClientResult<()> {
        let mut buf = UnpackBuffer::new(buf);

        let header = Header::unpack(&mut buf)?;
        // Check transaction ID to match. Implement fn accept::accept_as_client

        let message = Message::unpack(&mut buf, header)?;

        println!("{:?}", message);

        Ok(())
    }
}

pub struct ClientBuilder {
    network: Network,
    bind_timeout: time::Duration,
    read_timeout: time::Duration,
    write_timeout: time::Duration,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            network: Network::Udp,
            bind_timeout: time::Duration::from_secs(2),
            read_timeout: time::Duration::from_secs(2),
            write_timeout: time::Duration::from_secs(2),
        }
    }
}

impl ClientBuilder {
    pub async fn build(&self) -> Result<Client, ClientError> {
        let socket = match timeout(self.bind_timeout, UdpSocket::bind("0.0.0.0:0")).await {
            TimeoutResult::Timeout => return Err(ClientError::WriteTimeout(self.bind_timeout)),
            TimeoutResult::Error(err) => return Err(ClientError::IO(err)),
            TimeoutResult::Ok(socket) => socket,
        };

        Ok(Client {
            network: self.network,
            bind_timeout: self.bind_timeout,
            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,
            header_id: 1,
            socket,
        })
    }

    pub fn with_bind_timeout(&mut self, bind_timeout: time::Duration) -> &Self {
        self.bind_timeout = bind_timeout;
        self
    }

    pub fn with_read_timeout(&mut self, read_timeout: time::Duration) -> &Self {
        self.read_timeout = read_timeout;
        self
    }
}
