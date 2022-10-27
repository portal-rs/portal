use std::{net::SocketAddr, sync::Arc};

use tokio::net;

use crate::{
    packing::{self, UnpackBuffer, Unpackable},
    resolver::{self, ToResolver},
    server::accept,
    types::dns::{Header, Message},
};

pub struct Session {
    pub socket: Arc<net::UdpSocket>,
    pub addr: SocketAddr,
}

pub async fn handle(buf: &[u8], session: Session, res: Arc<resolver::Resolver>) {
    // Create an unpack buffer which keeps track of the offset automatically
    let mut buf = UnpackBuffer::new(buf);

    // Unpack DNS header data
    let header = match Header::unpack(&mut buf) {
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
            let message = match Message::unpack(&mut buf, header) {
                Ok(msg) => msg,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            handle_accept(message, session, res)
        }
        accept::Action::Reject => todo!(),
        accept::Action::Ignore => todo!(),
        accept::Action::NoImpl => todo!(),
    }
    .await;
}

async fn handle_accept(message: Message, session: Session, res: Arc<resolver::Resolver>) {
    // TODO (Techassi): Lookup in filter engine

    // TODO (Techassi): Lookup in cache

    // TODO (Techassi): Look for custom DNS records

    // Resolve via resolver
    let records = match res.resolve(&message) {
        Ok(recs) => recs,
        Err(_) => todo!(),
    };

    println!("{:#?} {}", message, records);
}
