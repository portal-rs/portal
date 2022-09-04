use std::net::SocketAddr;
use std::sync::Arc;

use tokio::{self, net};

use crate::{config, constants, packing, server::error::ServerError, server::network::Network};

pub mod error;
pub mod network;

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

        // Setup shared data here

        // Either start the UDP socket or TCP listener
        match self.network {
            Network::Tcp => todo!(),
            Network::Udp => {
                let socket = match net::UdpSocket::bind(self.addr_port).await {
                    Ok(socket) => socket,
                    Err(err) => {
                        return Err(ServerError::new(format!(
                            "Failed to bind UDP socket: {}",
                            err
                        )))
                    }
                };

                let socket = Arc::new(socket);
                let mut data = [0u8; constants::udp::UDP_MIN_MESSAGE_SIZE];

                loop {
                    let (len, addr) = match socket.recv_from(&mut data).await {
                        Ok(result) => result,
                        Err(err) => {
                            // TODO (Techassi): Log this
                            println!("{}", err);
                            continue;
                        }
                    };

                    let sender = socket.clone();

                    tokio::spawn(async move {
                        handle_udp(data[..len].to_vec(), addr, sender).await;
                    });
                }
            }
        }
    }
}

async fn handle_udp(data: Vec<u8>, addr: SocketAddr, socket: Arc<net::UdpSocket>) {
    println!(
        "Received {} bytes from {}:\n{:02X?}",
        data.len(),
        addr,
        data
    );

    // Unpack DNS header data
    let (header, offset) = match packing::unpack_header(&data) {
        Ok(result) => result,
        Err(_) => todo!(),
    };

    // Now we unpack the complete DNS message
    let message = match packing::unpack_message(header.clone(), data, offset) {
        Ok(msg) => msg,
        Err(_) => todo!(),
    };

    println!("{:#?}, {}", header, offset);
}
