use std::net::{IpAddr, SocketAddr};

pub struct Sockets(Vec<SocketAddr>);

impl From<(Vec<IpAddr>, u16)> for Sockets {
    fn from(value: (Vec<IpAddr>, u16)) -> Self {
        let (ips, port) = value;
        let mut socket_addrs = Vec::new();

        for ip in ips {
            socket_addrs.push(SocketAddr::new(ip, port));
        }

        Self(socket_addrs)
    }
}

impl From<SocketAddr> for Sockets {
    fn from(value: SocketAddr) -> Self {
        Self(vec![value])
    }
}

impl Sockets {
    pub fn iter(&self) -> SocketsIter<'_> {
        SocketsIter {
            sockets: &self.0,
            index: 0,
        }
    }

    pub fn iter_v4(&self) -> Vec<&SocketAddr> {
        self.iter().filter(|s| s.is_ipv4()).collect()
    }

    pub fn iter_v6(&self) -> Vec<&SocketAddr> {
        self.iter().filter(|s| s.is_ipv6()).collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct SocketsIter<'a> {
    sockets: &'a Vec<SocketAddr>,
    index: usize,
}

impl<'a> Iterator for SocketsIter<'a> {
    type Item = &'a SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.sockets.len() {
            return None;
        }

        let current = self.sockets.get(self.index);
        self.index += 1;

        current
    }
}
