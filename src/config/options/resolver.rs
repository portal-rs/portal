use std::net::{AddrParseError, SocketAddr};

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
    pub hint_path: String,
    pub mode: ResolveMode,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct RawResolverOptions {
    pub cache_enabled: bool,
    pub max_expire: usize,
    pub hint_path: String,
    pub upstream: String,
    pub mode: String,
}

impl Default for RawResolverOptions {
    fn default() -> Self {
        return Self {
            cache_enabled: true,
            max_expire: 300,
            hint_path: String::from(""),
            upstream: String::from(""),
            mode: String::from("r"),
        };
    }
}

impl RawResolverOptions {
    pub fn validate(&self) -> Result<ResolverOptions, ResolverOptionError> {
        let upstream: SocketAddr = self.upstream.parse()?;
        let mode: ResolveMode = self.mode.parse()?;

        Ok(ResolverOptions {
            upstream,
            cache_enabled: self.cache_enabled,
            max_expire: self.max_expire,
            hint_path: self.hint_path.clone(),
            mode,
        })
    }
}
