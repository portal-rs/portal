use std::{net::SocketAddr, sync::Arc};

use tokio::net;

use crate::{
    constants, packing,
    server::{accept, error::ServerError},
    types::dns::Message,
};

/// Start the UDP socket listener. This handles imcoming UDP data.
pub async fn serve(addr_port: SocketAddr) -> Result<(), ServerError> {
    let socket = match net::UdpSocket::bind(addr_port).await {
        Ok(socket) => socket,
        Err(err) => {
            return Err(ServerError::new(format!(
                "Failed to bind UDP socket: {}",
                err
            )))
        }
    };

    let socket = Arc::new(socket);
    let mut data = [0u8; constants::udp::MIN_MESSAGE_SIZE];

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
            handle(data[..len].to_vec(), addr, sender).await;
        });
    }
}

async fn handle(data: Vec<u8>, addr: SocketAddr, socket: Arc<net::UdpSocket>) {
    // Unpack DNS header data
    let (header, offset) = match packing::unpack_header(&data) {
        Ok(result) => result,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // Decide if the server should accept the message. This is done by looking
    // at some basic DNS header checks.
    match accept::should_accept(&header).await {
        accept::Action::Accept => {
            let message = match packing::unpack_message(header, data, offset) {
                Ok(msg) => msg,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            println!("{:#?}", message);
            handle_accept(message, addr, socket)
        }
        accept::Action::Reject => todo!(),
        accept::Action::Ignore => todo!(),
        accept::Action::NoImpl => todo!(),
    }
    .await;
}

async fn handle_accept(message: Message, addr: SocketAddr, socket: Arc<net::UdpSocket>) {
    println!("{:#?}", message);
}
