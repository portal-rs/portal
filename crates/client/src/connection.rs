use std::sync::Arc;

use tokio::net::UdpSocket;

pub struct Connection {
    socket: Arc<UdpSocket>,
    write_timeout: u64,
    read_timeout: u64,
    id: u16,
}

impl Connection {}
