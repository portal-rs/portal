use thiserror::Error;

use crate::client::ClientError;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Client error {0}")]
    ClientError(#[from] ClientError),

    #[error("No answer")]
    NoAnswer,

    #[error("No question")]
    NoQuestion,

    #[error("No SOA record")]
    NoSoaRecord,

    #[error("No glue records found")]
    NoGlueRecords,

    #[error("No more DNS server target IPs left")]
    NoMoreTargets,
}
