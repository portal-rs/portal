use std::net::SocketAddr;

use async_trait::async_trait;

use crate::{server::send::Sendable, types::dns::Message};

pub enum ResponseError {}

pub struct Response {
    message: Message,
    addr: SocketAddr,
}

pub trait ResponseHandler {
    fn send_response(response: Response, socket: impl Sendable) -> Result<(), ResponseError>;
}
