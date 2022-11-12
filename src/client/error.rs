use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Failed to bind socket")]
    Bind(#[from] std::io::Error),
}
