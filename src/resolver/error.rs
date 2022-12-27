use thiserror::Error;

use crate::client::ClientError;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Client error")]
    ClientError(#[from] ClientError),

    #[error("No answer")]
    NoAnswer,

    #[error("No SOA record")]
    NoSoaRecord,
}
