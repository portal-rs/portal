use portal_client::{Client, ClientError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BencherError {
    #[error("Client error: {0}")]
    ClientError(#[from] ClientError),
}

pub struct Bencher {
    client: Client,
}

impl Bencher {
    pub async fn new() -> Result<Self, BencherError> {
        let client = Client::new().await?;

        Ok(Self { client })
    }

    pub fn new_with_client(client: Client) -> Self {
        Self { client }
    }
}
