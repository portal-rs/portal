use std::{io, net::SocketAddr};

use async_trait::async_trait;
use tokio::net::UdpSocket;

#[async_trait]
pub trait Sendable {
    async fn send_to(&self, buf: &[u8], addr: SocketAddr) -> io::Result<usize>;
}

pub struct UdpDnsSocket(UdpSocket);

#[async_trait]
impl Sendable for UdpDnsSocket {
    async fn send_to(&self, buf: &[u8], addr: SocketAddr) -> io::Result<usize> {
        self.0.send_to(buf, addr).await
    }
}

impl UdpDnsSocket {
    pub fn new(socket: UdpSocket) -> Self {
        Self(socket)
    }

    pub fn inner(&self) -> &UdpSocket {
        &self.0
    }
}
