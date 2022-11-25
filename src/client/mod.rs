use std::time;

use rand;
use tokio::net::{ToSocketAddrs, UdpSocket};

use crate::{
    packing::{PackBuffer, Packable},
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
    ///
    /// ### Example
    ///
    /// ```
    ///
    ///
    ///
    ///
    ///
    /// ```
    pub async fn query(
        &mut self,
        name: Name,
        class: Class,
        ty: Type,
        addr: impl ToSocketAddrs,
    ) -> ClientResult<()> {
        let id = self.get_and_refresh_header_id();
        let header = Header::new(id);
        let mut message = Message::new_with_header(header);

        message.add_question(Question { name, class, ty });

        let mut buf = PackBuffer::new();
        if let Err(err) = message.pack(&mut buf) {
            return Err(ClientError::WriteToBuf(err));
        }

        match timeout(self.write_timeout, self.socket.send_to(buf.bytes(), addr)).await {
            TimeoutResult::Timeout => return Err(ClientError::WriteTimeout(self.write_timeout)),
            TimeoutResult::Error(err) => return Err(ClientError::IO(err)),
            TimeoutResult::Ok(_) => {}
        }

        loop {}
    }

    /// Gets the current header ID and generates a new random one.
    fn get_and_refresh_header_id(&mut self) -> u16 {
        let id = self.header_id;
        self.header_id = rand::random::<u16>();

        return id;
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
        let socket = match timeout(self.bind_timeout, UdpSocket::bind("127.0.0.1:0")).await {
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
