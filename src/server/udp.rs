use std::sync::Arc;

use binbuf::prelude::*;

use crate::{
    resolver,
    server::accept,
    types::{
        dns::{Header, Message},
        udp::Session,
    },
};

pub async fn handle(buf: &[u8], session: Session, res: Arc<impl resolver::ToResolver>) {
    // Create an unpack buffer which keeps track of the offset automatically
    let mut buf = ReadBuffer::new(buf);

    // Unpack DNS header data
    let header = match Header::read_be(&mut buf) {
        Ok(result) => result,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // Decide if the server should accept the message. This is done by looking
    // at some basic DNS header checks.
    match accept::should_accept(&header).await {
        accept::Action::Accept => {
            let mut message = match Message::read::<BigEndian>(&mut buf, header) {
                Ok(msg) => msg,
                Err(err) => {
                    println!("{err}");
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
    let mut records = match res.resolve(message).await {
        Ok(recs) => recs,
        Err(err) => {
            println!("{err}");
            // TODO (Techassi): Handle error
            return;
        }
    };

    // NOTE (Techassi): Is this the best way to do this? We don't care about
    // the original rdlens, but we should keep them initially if we actually
    // need them. Normalizing them here seems the correct way until we
    // support writing back compressed names / messages.
    records.normalize_rdlens();
    message.add_query_result(&mut records);
    handle_response(message, session).await;
}

async fn handle_response(message: &mut Message, session: Session) {
    let mut buf = WriteBuffer::new();

    // Set some response specific values in the message
    message.set_is_response(true);
    message.set_rec_avail(true);

    // println!("{message:?}");

    if let Err(err) = message.write::<BigEndian>(&mut buf) {
        // TODO (Techassi): Return message with RCODE 2
        println!("{err}");
        return;
    }

    // TODO (Techassi): Think about where we should handle the IO errors
    if let Err(err) = session.socket.send_to(buf.bytes(), session.addr).await {
        println!("{err}");
    };
}
