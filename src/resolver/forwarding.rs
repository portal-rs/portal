use std::net::SocketAddr;

use async_trait::async_trait;

use crate::{
    client::Client,
    errors::ResolverError,
    resolver::{ResolveResult, ResultRecords, ToResolver},
    types::dns::{Message, ToQuery},
};

pub struct ForwardingResolver {
    client: Client,
    addr: SocketAddr,
}

#[async_trait]
impl ToResolver for ForwardingResolver {
    async fn resolve(&self, message: &Message) -> ResolveResult {
        self.resolve_raw((
            message.question[0].name.clone(),
            message.question[0].ty,
            message.question[0].class,
        ))
        .await
    }

    async fn resolve_raw<Q: ToQuery>(&self, query: Q) -> ResolveResult {
        match self.client.query(query, self.addr).await {
            Ok(msg) => Ok(ResultRecords::from(msg)),
            Err(err) => Err(ResolverError::ClientError(err)),
        }
    }

    // async fn lookup<Q: ToQuery>(&self, query: Q) -> ResolveResult {
    //     todo!()
    // }

    // async fn refresh<Q: ToQuery>(&self, query: Q) {
    //     todo!()
    // }
}

impl ForwardingResolver {
    pub async fn new(addr: SocketAddr) -> Result<Self, ResolverError> {
        let client = match Client::new().await {
            Ok(client) => client,
            Err(err) => return Err(ResolverError::ClientError(err)),
        };

        Ok(Self { client, addr })
    }
}
