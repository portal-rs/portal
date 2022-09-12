use std::{
    sync::{Arc, Mutex},
    time,
};

use tokio::net::UdpSocket;

use crate::{client::error::ClientError, utils::network::Network};

mod error;

pub struct Client {
    network: Network,
    dial_timeout: time::Duration,
    read_timeout: time::Duration,
    write_timeout: time::Duration,
    header_id: Mutex<u16>,
    socket: Arc<UdpSocket>,
}

pub struct ClientBuilder {
    network: Network,
    dial_timeout: time::Duration,
    read_timeout: time::Duration,
    write_timeout: time::Duration,
}

impl Client {
    /// Tries to create a new DNS [`Client`] with default settings. This client
    /// tries to bind to a random unused port for communication with other DNS
    /// servers. This is achieved by binding to port `0`, which instructs the
    /// OS to bind to an unused port.
    pub async fn new() -> Result<Client, ClientError> {
        let socket = match UdpSocket::bind("127.0.0.1:0").await {
            Ok(sock) => sock,
            Err(err) => {
                return Err(ClientError::new(format!(
                    "Failed to open UDP socket for DNS client: {}",
                    err
                )))
            }
        };
        let socket = Arc::new(socket);

        let client = Client {
            network: Network::Udp,
            dial_timeout: time::Duration::from_secs(2),
            read_timeout: time::Duration::from_secs(2),
            write_timeout: time::Duration::from_secs(2),
            header_id: Mutex::new(1),
            socket,
        };

        Ok(client)
    }

    pub async fn dial() {}
}
