use std::{net::SocketAddr, sync::Arc};

use tokio::net;

pub struct Session {
    pub socket: Arc<net::UdpSocket>,
    pub addr: SocketAddr,
}
