use std::sync::Arc;

use tokio::{self, net};

use crate::{
    config, constants,
    resolver::{ResolveMode, Resolver},
    types::udp::Session,
    utils::Network,
};

mod accept;
mod error;
mod tcp;
mod udp;

pub use error::*;

pub struct Server {
    addr_port: std::net::SocketAddr,
    resolve_mode: ResolveMode,
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
            resolve_mode: ResolveMode::Recursive,
            network: Network::Tcp,
            running: false,
        }
    }
}

impl Server {
    pub fn new(cfg: config::Config) -> Result<Self, ServerError> {
        let addr_port: std::net::SocketAddr = match cfg.server.address.parse() {
            Ok(addr) => addr,
            Err(err) => return Err(ServerError::InvalidAddress(err.to_string())),
        };

        let network = match Network::parse(cfg.server.network) {
            Ok(net) => net,
            Err(err) => return Err(ServerError::InvalidNetwork(err.to_string())),
        };

        let resolve_mode = match ResolveMode::parse(cfg.resolver.mode) {
            Ok(mode) => mode,
            Err(err) => return Err(ServerError::InvalidResolverMode(err.to_string())),
        };

        return Ok(Self {
            addr_port,
            resolve_mode,
            network,
            running: false,
        });
    }

    #[tokio::main]
    pub async fn run(&mut self) -> Result<(), ServerError> {
        if self.running {
            return Err(ServerError::AlreadyRunning);
        }
        self.running = true;

        // Either start the UDP socket or TCP listener
        match self.network {
            Network::Tcp => todo!(),
            Network::Udp => self.run_udp().await,
        }
    }

    async fn run_udp(&self) -> Result<(), ServerError> {
        let socket = match net::UdpSocket::bind(self.addr_port).await {
            Ok(socket) => socket,
            Err(err) => return Err(ServerError::Bind(err.to_string())),
        };

        let resolver = match Resolver::new_from(self.resolve_mode.clone()).await {
            Ok(resolver) => resolver,
            Err(_) => todo!(),
        };

        let resolver = Arc::new(resolver);
        let socket = Arc::new(socket);

        loop {
            // Wait until the socket is readable, this can produce a false positive
            socket.readable().await?;

            let mut buf = [0u8; constants::udp::MIN_MESSAGE_SIZE];
            let (len, addr) = match socket.recv_from(&mut buf).await {
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

            let resolver = resolver.clone();

            let session = Session {
                socket: socket.clone(),
                addr,
            };

            tokio::spawn(async move {
                udp::handle(&buf[..len], session, resolver).await;
            });
        }
    }
}
