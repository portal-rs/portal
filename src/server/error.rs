use thiserror::Error;

use crate::errors::ResolverError;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Invalid resolver mode - expected r/f/i ({0})")]
    InvalidResolverMode(String),

    #[error("Invalid bind address ({0})")]
    InvalidAddress(String),

    #[error("Invalid network - expected udp/tcp ({0})")]
    InvalidNetwork(String),

    #[error("Failed to start server, already running")]
    AlreadyRunning,

    #[error("Resolver error: {0}")]
    ResolverError(#[from] ResolverError),

    #[error("Failed to bind socket ({0})")]
    Bind(String),

    #[error("IO error")]
    IO(#[from] std::io::Error),
}
