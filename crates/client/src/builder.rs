use std::{collections::HashSet, sync::Arc, time::Duration};

use portal_common::{timeout, IpVersion, TimeoutResult};
use portal_proto::constants::MIN_MESSAGE_SIZE;
use tokio::net::UdpSocket;

use crate::{Client, ClientError};

// use crate::{
//     constants::udp::MIN_MESSAGE_SIZE,
//     types::ip_version::IpVersion,
//     utils::{timeout, TimeoutResult},
//     Client, ClientError,
// };

pub struct ClientBuilder {
    ip_version: IpVersion,
    write_timeout: u64,
    buffer_size: usize,
    bind_timeout: u64,
    read_timeout: u64,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            ip_version: IpVersion::default(),
            buffer_size: MIN_MESSAGE_SIZE,
            write_timeout: 2,
            bind_timeout: 2,
            read_timeout: 2,
        }
    }
}

impl ClientBuilder {
    pub async fn build(&self) -> Result<Client, ClientError> {
        let bind_timeout = Duration::from_secs(self.bind_timeout);
        let bind_address = match self.ip_version {
            IpVersion::Both | IpVersion::V6 => "[::]:0",
            IpVersion::V4 => "0.0.0.0:0",
        };

        let socket = match timeout(bind_timeout, UdpSocket::bind(bind_address)).await {
            TimeoutResult::Timeout => return Err(ClientError::WriteTimeout(bind_timeout)),
            TimeoutResult::Error(err) => return Err(ClientError::IO(err)),
            TimeoutResult::Ok(socket) => socket,
        };

        Ok(Client {
            active_ids: Arc::new(HashSet::new()),
            write_timeout: self.write_timeout,
            read_timeout: self.read_timeout,
            buffer_size: self.buffer_size,
            socket: Arc::new(socket),
        })
    }

    /// Customize the socket bind timeout.
    pub fn with_bind_timeout(&mut self, bind_timeout: u64) -> &mut Self {
        self.bind_timeout = bind_timeout;
        self
    }

    /// Customize the socket read timeout.
    pub fn with_read_timeout(&mut self, read_timeout: u64) -> &mut Self {
        self.read_timeout = read_timeout;
        self
    }

    /// Customize the socket write timeout.
    pub fn with_write_timeout(&mut self, write_timeout: u64) -> &mut Self {
        self.write_timeout = write_timeout;
        self
    }

    /// Customize the buffer size for receiving DNS responses. An minimum size
    /// if 512 octets is enforced. Providing a buffer size below this size
    /// will result in a buffer size of 512 octets.
    pub fn with_buffer_size(&mut self, buffer_size: usize) -> &mut Self {
        self.buffer_size = if buffer_size < MIN_MESSAGE_SIZE {
            MIN_MESSAGE_SIZE
        } else {
            buffer_size
        };

        self
    }

    /// Customize the bind address based on the desired IP version. When either
    /// [`IpVersion::Both`] or [`IpVersion::V6`] is provided, the client binds
    /// to `[::]:0` and `0.0.0.0:0` when [`IpVersion::V4`] is used.
    pub fn with_ip_version<T>(&mut self, ip_version: T) -> &mut Self
    where
        T: Into<IpVersion>,
    {
        self.ip_version = ip_version.into();
        self
    }
}
