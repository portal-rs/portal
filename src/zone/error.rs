use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZoneError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Failed to parse zone file: {0}")]
    ParseError(String),
}
