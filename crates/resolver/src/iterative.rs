use async_trait::async_trait;

use crate::{
    resolver::{ResolveResult, ToResolver},
    types::dns::ToQuery,
};

pub struct IterativeResolver {}

#[async_trait]
impl ToResolver for IterativeResolver {
    async fn resolve(&self, message: &crate::types::dns::Message) -> ResolveResult {
        todo!()
    }

    async fn resolve_raw<Q: ToQuery>(&self, query: Q) -> ResolveResult {
        todo!()
    }

    // async fn lookup<Q: ToQuery>(&self, query: Q) -> ResolveResult {
    //     todo!()
    // }

    // async fn refresh<Q: ToQuery>(&self, query: Q) {
    //     todo!()
    // }
}
