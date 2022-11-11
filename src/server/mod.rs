use std::sync::Arc;

use tokio::{self, net};

use crate::{
    config, constants,
    errors::{AppError, AppErrorVariant, ServerError},
    resolver::{ResolveMode, Resolver},
    utils::network::Network,
};

pub mod error;

mod accept;
mod tcp;
mod udp;

pub struct Server {
    addr_port: std::net::SocketAddr,
    ancillary_size: usize,
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
            ancillary_size: 0,
            resolve_mode: ResolveMode::Recursive,
            network: Network::Tcp,
            running: false,
        }
    }
}

impl Server {
    pub fn new(cfg: config::Config) -> Result<Self, AppError> {
        let addr_port: std::net::SocketAddr = match cfg.server.address.parse() {
            Ok(addr) => addr,
            Err(err) => {
                return Err(AppError::new(
                    err.to_string(),
                    AppErrorVariant::ServerError(ServerError::InvalidAddress),
                ))
            }
        };

        let network = match Network::parse(cfg.server.network) {
            Ok(net) => net,
            Err(err) => {
                return Err(AppError::new(
                    err.to_string(),
                    AppErrorVariant::ServerError(ServerError::InvalidNetwork),
                ))
            }
        };

        let resolve_mode = match ResolveMode::parse(cfg.resolver.mode) {
            Ok(mode) => mode,
            Err(err) => {
                return Err(AppError::new(
                    err.to_string(),
                    AppErrorVariant::ServerError(ServerError::InvalidResolverMode),
                ))
            }
        };

        return Ok(Self {
            addr_port,
            ancillary_size: 0,
            resolve_mode,
            network,
            running: false,
        });
    }

    #[tokio::main]
    pub async fn run(&mut self) -> Result<(), AppError> {
        if self.running {
            return Err(AppError::new(
                "This server instance is already running. It can be restarted",
                AppErrorVariant::ServerError(ServerError::AlreadyRunning),
            ));
        }
        self.running = true;

        // Either start the UDP socket or TCP listener
        match self.network {
            Network::Tcp => todo!(),
            Network::Udp => self.run_udp().await,
        }
    }

    async fn run_udp(&self) -> Result<(), AppError> {
        let socket = match net::UdpSocket::bind(self.addr_port).await {
            Ok(socket) => socket,
            Err(err) => {
                return Err(AppError::new(
                    err.to_string(),
                    AppErrorVariant::ServerError(ServerError::BindFailure),
                ))
            }
        };

        let resolver = match Resolver::new_from(self.resolve_mode.clone()).await {
            Ok(resolver) => resolver,
            Err(_) => todo!(),
        };

        let resolver = Arc::new(resolver);
        let socket = Arc::new(socket);

        loop {
            let mut buf = vec![0u8; constants::udp::MIN_MESSAGE_SIZE];
            let (len, addr) = match socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(err) => {
                    // TODO (Techassi): Log this
                    println!("{}", err);
                    continue;
                }
            };
            buf.resize(len, 0);

            let resolver = resolver.clone();

            let session = udp::Session {
                socket: socket.clone(),
                addr,
            };

            tokio::spawn(async move {
                udp::handle(buf.as_slice(), session, resolver).await;
            });
        }
    }
}
