use std::time;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("IO error")]
    IO(#[from] std::io::Error),

    #[error("Failed to acquire mutex lock")]
    WriteToBuf(#[from] crate::error::ProtocolError),

    #[error("Failed to send query message: Only send partial data")]
    SendPartial,

    #[error("Bind error: Failed to create and bind UDP socket after {0:?}")]
    BindTimeout(time::Duration),

    #[error("Writing to socket timed out after {0:?}")]
    WriteTimeout(time::Duration),
}
