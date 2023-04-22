use thiserror::Error;

use crate::{tree::TreeError, RDataParseError};

#[derive(Debug, Error)]
pub enum ZoneError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Failed to parse zone file: {0}")]
    ParseError(String),

    #[error("Tree error: {0}")]
    TreeError(#[from] TreeError),
}

impl From<RDataParseError> for ZoneError {
    fn from(value: RDataParseError) -> Self {
        Self::ParseError(value.to_string())
    }
}
