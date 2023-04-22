use std::time;

use portal_proto::{HeaderError, MessageError};
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Message error: {0}")]
    MessageError(#[from] MessageError),

    #[error("Header error: {0}")]
    HeaderError(#[from] HeaderError),

    #[error("Failed to send query message: Only send partial data")]
    SendPartial,

    #[error("Bind error: Failed to create and bind UDP socket after {0:?}")]
    BindTimeout(time::Duration),

    #[error("Writing to socket timed out after {0:?}")]
    WriteTimeout(time::Duration),

    #[error("Reading from socket timed out after {0:?}")]
    ReadTimeout(time::Duration),

    #[error("Runtime error: {0}")]
    RuntimeError(#[from] JoinError),
}
