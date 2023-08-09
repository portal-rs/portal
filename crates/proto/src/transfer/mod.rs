use std::net::SocketAddr;

use crate::Message;

mod background;
mod handler;
mod multiplexer;
mod protocol;
mod transport;

pub use background::*;
pub use handler::*;
pub use multiplexer::*;
pub use protocol::*;
pub use transport::*;

pub trait RequestExt {
    fn target(&self) -> SocketAddr;
    fn message(&self) -> &Message;
}

pub struct Request {
    target_socket_addr: SocketAddr,
    message: Message,
}

impl RequestExt for Request {
    fn target(&self) -> SocketAddr {
        self.target_socket_addr
    }

    fn message(&self) -> &Message {
        &self.message
    }
}

impl Request {
    pub fn new(message: Message, target: SocketAddr) -> Self {
        Self {
            target_socket_addr: target,
            message,
        }
    }
}
