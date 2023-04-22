use thiserror::Error;

use crate::config::{ResolverOptionError, ServerOptionError};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error while deserializing TOML")]
    Deserialize(#[from] toml::de::Error),

    #[error("Error while serializing TOML")]
    Serialize,

    #[error("Error while writing TOML config file")]
    Write,

    #[error("Error while reading TOML config file")]
    Read(#[from] std::io::Error),

    #[error("Error while validating resolver options: {0}")]
    ResolverOptionError(#[from] ResolverOptionError),

    #[error("Error while validating server options: {0}")]
    ServerOptionError(#[from] ServerOptionError),
}
