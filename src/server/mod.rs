use std::sync::Arc;

use tokio::{self, net};

use crate::{
    config::Config,
    constants,
    resolver::{ForwardingResolver, RecursiveResolver, ResolveMode, Resolver},
    types::udp::Session,
    utils::Network,
};

mod accept;
mod error;
mod handler;
mod request;
mod response;
mod send;
mod tcp;
mod udp;

pub use error::*;

pub struct Server {
    config: Config,
    running: bool,
}

impl Server {
    pub fn new(cfg: Config) -> Self {
        Self {
            config: cfg,
            running: false,
        }
    }

    #[tokio::main]
    pub async fn run(&mut self) -> Result<(), ServerError> {
        if self.running {
            return Err(ServerError::AlreadyRunning);
        }
        self.running = true;

        // Either start the UDP socket or TCP listener
        match self.config.server.network {
            Network::Tcp => todo!(),
            Network::Udp => self.run_udp().await,
        }
    }

    async fn run_udp(&self) -> Result<(), ServerError> {
        let socket = match net::UdpSocket::bind(self.config.server.address).await {
            Ok(socket) => socket,
            Err(err) => return Err(ServerError::Bind(err.to_string())),
        };

        let resolver: Resolver = match self.config.resolver.mode {
            ResolveMode::Recursive => {
                RecursiveResolver::new(self.config.resolver.hint_file_path.clone())
                    .await?
                    .into()
            }
            ResolveMode::Iterative => todo!(),
            ResolveMode::Forwarding => ForwardingResolver::new(self.config.resolver.upstream)
                .await?
                .into(),
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
