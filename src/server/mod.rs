use tokio;

use crate::{config, server::error::ServerError, server::network::Network};

mod accept;
pub mod error;
pub mod network;
mod udp;

pub struct Server {
    addr_port: std::net::SocketAddr,
    ancillary_size: usize,
    network: Network,
    running: bool,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            addr_port: std::net::SocketAddr::new(
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                53,
            ),
            ancillary_size: 0,
            network: Network::Tcp,
            running: false,
        }
    }
}

impl Server {
    pub fn new(cfg: config::Config) -> Result<Self, ServerError> {
        let addr_port: std::net::SocketAddr = match cfg.server.address.parse() {
            Ok(addr) => addr,
            Err(err) => {
                return Err(ServerError::new(format!(
                    "Failed to parse server address: {}",
                    err
                )))
            }
        };

        let network = match Network::parse(cfg.server.network) {
            Ok(net) => net,
            Err(err) => {
                return Err(ServerError::new(format!(
                    "Failed to parse server network: {}",
                    err
                )))
            }
        };

        return Ok(Self {
            addr_port,
            ancillary_size: 0,
            network,
            running: false,
        });
    }

    #[tokio::main]
    pub async fn run(&mut self) -> Result<(), ServerError> {
        if self.running {
            return Err(ServerError::new("Server is already running"));
        }
        self.running = true;

        // Either start the UDP socket or TCP listener
        match self.network {
            Network::Tcp => todo!(),
            Network::Udp => udp::serve(self.addr_port).await,
        }
    }
}
