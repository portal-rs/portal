use std::net::{AddrParseError, SocketAddr};

use portal_common::{Network, NetworkError};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerOptionError {
    #[error("Bind addr parse error: {0}")]
    AddrParseError(#[from] AddrParseError),

    #[error("Network parse error: {0}")]
    NetworkParseError(#[from] NetworkError),
}

pub struct ServerOptions {
    pub cache_enabled: bool,
    pub address: SocketAddr,
    pub network: Network,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct RawServerOptions {
    pub cache_enabled: bool,
    pub address: String,
    pub network: String,
}

impl Default for RawServerOptions {
    fn default() -> Self {
        Self {
            cache_enabled: true,
            address: String::from("127.0.0.1:53"),
            network: String::from("udp"),
        }
    }
}

impl RawServerOptions {
    pub fn validate(&self) -> Result<ServerOptions, ServerOptionError> {
        let address: SocketAddr = self.address.parse()?;
        let network: Network = self.network.parse()?;

        Ok(ServerOptions {
            cache_enabled: self.cache_enabled,
            address,
            network,
        })
    }
}
