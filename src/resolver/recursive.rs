use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::{
    client::Client,
    macros::cast_or,
    resolver::{ResolveResult, ResolverError, ToResolver},
    types::{
        dns::{Message, Query, ToQuery},
        rr::RData,
    },
};

use async_trait::async_trait;

pub struct RecursiveResolver {
    client: Client,
}

#[async_trait]
impl ToResolver for RecursiveResolver {
    async fn resolve(&self, message: &Message) -> ResolveResult {
        self.resolve_raw(message).await
    }

    async fn resolve_raw<Q: ToQuery>(&self, query: Q) -> ResolveResult {
        let query = query.to_query();
        let mut target = self.hint();

        // TODO (Techassi): Introduce a state machine here
        loop {
            let message = self
                .client
                .query(query.clone(), SocketAddr::new(target, 53))
                .await?;

            // We got at least one answer. We can immediatly return these.
            if message.ancount() > 0 {
                return Ok(message.into());
            }

            // We received no NS records. That's bad.
            if message.nscount() == 0 {
                return Err(ResolverError::NoAnswer);
            }

            // We can ask the original / primary DNS server. This involves
            // looking up the IP address for the provided domain name. If we
            // were able to retrieve the IP, we can continue this loop with
            // the updated target IP address.
            if message.is_soa() {
                let soa = match message.get_soa_record() {
                    Some(soa) => soa,
                    None => return Err(ResolverError::NoSoaRecord),
                };

                let soa_query = Query::new(soa.get_mname().clone(), query.ty, query.class);
                let _results = self.resolve_raw(soa_query).await?;
            }

            // At this step there should be some "glue" records. These records
            // provide NS records in the authority section. NS RRs contain a
            // domain name. To avoid resolving this name, most DNS servers
            // provide A and AAAA records for these domain names in the
            // additional records section.
            self.find_glue_records(&message).await
        }
    }
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

    pub async fn find_glue_records(&self, message: &Message) {
        for ns_record in &message.authorities {
            let ns_name = cast_or!(ns_record.get_rdata(), RData::NS, continue);
        }
    }

    pub fn hint(&self) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(198, 41, 0, 4))
    }
}
