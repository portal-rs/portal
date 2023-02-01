use std::net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr};

use serde::Deserialize;
use thiserror::Error;

use crate::resolver::{ResolveMode, ResolveModeError};

#[derive(Debug, Error)]
pub enum ResolverOptionError {
    #[error("Upstream addr parse error: {0}")]
    AddrParseError(#[from] AddrParseError),

    #[error("Resolve mode parse error: {0}")]
    ResolveModeParseError(#[from] ResolveModeError),
}

pub struct ResolverOptions {
    pub upstream: SocketAddr,
    pub cache_enabled: bool,
    pub max_expire: usize,
    pub hint_file_path: String,
    pub mode: ResolveMode,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct RawResolverOptions {
    pub cache_enabled: bool,
    pub max_expire: usize,
    pub hint_file_path: String,
    pub upstream: String,
    pub mode: String,
}

impl Default for RawResolverOptions {
    fn default() -> Self {
        Self {
            cache_enabled: true,
            max_expire: 300,
            hint_file_path: String::from(""),
            upstream: String::from(""),
            mode: String::from("r"),
        }
    }
}

impl RawResolverOptions {
    pub fn validate(&self) -> Result<ResolverOptions, ResolverOptionError> {
        let mode: ResolveMode = self.mode.parse()?;

        // Only parse the upstreqm addr when we use the forwarding resolver.
        // Otherwise fallback to 0.0.0.0:0
        let upstream = match mode {
            ResolveMode::Forwarding => self.upstream.parse()?,
            _ => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        };

        Ok(ResolverOptions {
            upstream,
            cache_enabled: self.cache_enabled,
            max_expire: self.max_expire,
            hint_file_path: self.hint_file_path.clone(),
            mode,
        })
    }
}
