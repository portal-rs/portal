use std::sync::Arc;

use crate::{
    packing::{PackBuffer, Packable, UnpackBuffer, Unpackable},
    resolver,
    server::accept,
    types::{
        dns::{Header, Message},
        udp::Session,
    },
};

pub async fn handle(buf: &[u8], session: Session, res: Arc<impl resolver::ToResolver>) {
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
            let mut message = match Message::unpack(&mut buf, header) {
                Ok(msg) => msg,
                Err(err) => {
                    println!("{}", err);
                    return;
                }
            };

            handle_accept(&mut message, session, res).await;
        }
        accept::Action::Reject => todo!(),
        accept::Action::Ignore => todo!(),
        accept::Action::NoImpl => todo!(),
    }
}

async fn handle_accept(
    message: &mut Message,
    session: Session,
    res: Arc<impl resolver::ToResolver>,
) {
    // TODO (Techassi): Lookup in filter engine

    // TODO (Techassi): Lookup in cache

    // TODO (Techassi): Look for custom DNS records

    // Resolve via resolver
    let mut records = match res.resolve(&message).await {
        Ok(recs) => recs,
        Err(_) => todo!(),
    };

    message.add_query_result(&mut records);
    handle_response(message, session).await;
}

async fn handle_response(message: &mut Message, session: Session) {
    let mut buf = PackBuffer::new();

    // Set some response specific values in the message
    message.set_is_response(true);
    message.set_rec_avail(true);

    println!("{:?}", message);

    if let Err(err) = message.pack(&mut buf) {
        // TODO (Techassi): Return message with RCODE 2
        println!("{}", err);
        return;
    }

    // TODO (Techassi): Think about where we should handle the IO errors
    if let Err(err) = session.socket.send_to(buf.bytes(), session.addr).await {
        println!("{}", err);
    };
}
