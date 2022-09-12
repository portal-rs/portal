use crate::{
    client::Client,
    resolver::{ResolveResult, ResolverError, ToResolver},
    types::dns::Message,
};

pub struct RecursiveResolver {
    client: Client,
}

impl RecursiveResolver {
    pub async fn new() -> Result<Self, ResolverError> {
        let client = match Client::new().await {
            Ok(client) => client,
            Err(_) => todo!(),
        };

        let resolver = Self { client };
        Ok(resolver)
    }
}

impl ToResolver for RecursiveResolver {
    fn resolve(&self, message: &Message) -> ResolveResult {
        todo!()
    }

    fn resolve_raw(&self, name: String, class: u16, typ: u16) -> ResolveResult {
        todo!()
    }

    fn lookup(&self, name: String, class: u16, typ: u16) -> ResolveResult {
        todo!()
    }

    fn refresh(&self, name: String, class: u16, typ: u16) {
        todo!()
    }
}
