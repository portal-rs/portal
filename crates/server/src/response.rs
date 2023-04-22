use std::net::SocketAddr;

use portal_proto::{transfer::Sendable, Message};

pub enum ResponseError {}

pub struct Response {
    message: Message,
    addr: SocketAddr,
}

pub trait ResponseHandler {
    fn send_response(response: Response, socket: impl Sendable) -> Result<(), ResponseError>;
}
