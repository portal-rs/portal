use std::net::{IpAddr, SocketAddr};

pub struct Sockets(Vec<SocketAddr>);

pub trait IntoSockets: Sized {
    fn into_sockets(self) -> Vec<SocketAddr>;
    fn into_ipv4_sockets(self) -> Vec<SocketAddr>;
    fn into_ipv6_sockets(self) -> Vec<SocketAddr>;
    fn into_filtered_sockets(self, use_ipv4: bool, use_ipv6: bool) -> Vec<SocketAddr> {
        if !use_ipv4 && !use_ipv6 {
            return IntoSockets::into_sockets(self);
        } else if !use_ipv4 && use_ipv6 {
            return IntoSockets::into_ipv6_sockets(self);
        } else {
            return IntoSockets::into_ipv4_sockets(self);
        }
    }
}

impl IntoSockets for (Vec<IpAddr>, u16) {
    fn into_sockets(self) -> Vec<SocketAddr> {
        let (ips, port) = self;
        let mut addrs = Vec::new();

        for ip in ips {
            addrs.push(SocketAddr::new(ip, port))
        }

        addrs
    }

    fn into_ipv4_sockets(self) -> Vec<SocketAddr> {
        let ips: Vec<IpAddr> = self.0.iter().copied().filter(|i| i.is_ipv4()).collect();
        IntoSockets::into_sockets((ips, self.1))
    }

    fn into_ipv6_sockets(self) -> Vec<SocketAddr> {
        let ips: Vec<IpAddr> = self.0.iter().copied().filter(|i| i.is_ipv6()).collect();
        IntoSockets::into_sockets((ips, self.1))
    }
}

impl IntoSockets for SocketAddr {
    fn into_sockets(self) -> Vec<SocketAddr> {
        vec![self]
    }

    fn into_ipv4_sockets(self) -> Vec<SocketAddr> {
        if self.is_ipv4() {
            return vec![self];
        }

        vec![]
    }

    fn into_ipv6_sockets(self) -> Vec<SocketAddr> {
        if self.is_ipv6() {
            return vec![self];
        }

        vec![]
    }
}
