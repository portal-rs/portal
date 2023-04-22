use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Mutex,
};

use async_trait::async_trait;
use portal_client::{Client, ClientError};
use portal_common::{cast, cast_or};
use portal_proto::{sockets::IntoSockets, Message, RData, RType, ToQuery, Zone};

use crate::{ResolveResult, ResolverError, ToResolver};

#[derive(Debug)]
pub struct Hint {
    ipv4_addr: Ipv4Addr,
    ipv6_addr: Ipv6Addr,
}

impl Default for Hint {
    fn default() -> Self {
        Self {
            ipv4_addr: Ipv4Addr::new(0, 0, 0, 0),
            ipv6_addr: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
        }
    }
}

pub trait IntoHints {
    fn into_hints(self) -> Vec<Hint>;
}

impl IntoHints for Zone {
    fn into_hints(self) -> Vec<Hint> {
        let mut hints = Vec::new();

        // Iterate through the root NS records
        for record in self.tree.root().records() {
            if *record.header().ty() != RType::NS {
                continue;
            }

            let rdata = cast!(record.rdata(), RData::NS).unwrap();
            let ns_node = self.tree.find_node(rdata.clone());

            let mut hint = Hint::default();

            // Use the NS record to find the associated A and AAAA records
            for ns_record in ns_node.unwrap().records() {
                let ty = *ns_record.header().ty();

                if ty != RType::A && ty != RType::AAAA {
                    continue;
                }

                // Save the IPv4 and IPv6 in the hint
                match ns_record.rdata() {
                    RData::A(ipv4_addr) => hint.ipv4_addr = *ipv4_addr,
                    RData::AAAA(ipv6_addr) => hint.ipv6_addr = *ipv6_addr,
                    _ => {}
                }
            }

            hints.push(hint);
        }

        hints
    }
}

pub struct RecursiveResolver {
    hint_index: Mutex<usize>,
    hints: Vec<Hint>,
    client: Client,
}

#[async_trait]
impl ToResolver for RecursiveResolver {
    async fn resolve(&self, message: &Message) -> ResolveResult {
        self.resolve_raw(message).await
    }

    async fn resolve_raw<Q: ToQuery>(&self, query: Q) -> ResolveResult {
        let mut target_candidates: Vec<IpAddr> = Vec::new();
        let query = query.to_query();

        target_candidates.push(self.hint());

        // TODO (Techassi): Introduce a state machine here
        loop {
            // Remove one of the target candidates and use it to send a DNS
            // query.
            let target = match target_candidates.pop() {
                Some(t) => t,
                None => return Err(ResolverError::NoMoreTargets),
            };

            // If we timeout on read, this is most likely a network related
            // issue, e.g. the target server is not responding. If this
            // happens we just continue the loop and remove the next target
            // candidate in line.
            let (message, _, _) = match self
                .client
                .query(query.clone(), SocketAddr::new(target, 53).into_sockets())
                .await
            {
                Ok(msg) => msg,
                Err(err) => match err {
                    ClientError::ReadTimeout(_) => continue,
                    _ => return Err(ResolverError::ClientError(err)),
                },
            };

            // We got at least one answer. We can immediately return these.
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

            // TODO (Techassi): Add support to handle SOA records
            // if message.is_soa() {
            //     let soa = match message.get_soa_record() {
            //         Some(soa) => soa,
            //         None => return Err(ResolverError::NoSoaRecord),
            //     };

            //     let soa_query = Query::new(soa.get_mname().clone(), query.ty, query.class);
            //     let _results = self.resolve_raw(soa_query).await?;
            // }

            // At this step there should be some "glue" records. These records
            // provide NS records in the authority section. NS RRs contain a
            // domain name. To avoid resolving this name, most DNS servers
            // provide A and AAAA records for these domain names in the
            // additional records section.
            if let Some(mut ip_addrs) = self.find_glue_records(&message).await {
                target_candidates.clear();
                target_candidates.append(&mut ip_addrs);
                continue;
            }

            // The DNS server didn't provide any glue records in the additional
            // section, bummer... We know have to look them up manually by
            // querying the root DNS servers again.

            // TODO (Techassi): We should query for multiple NS servers in
            // parallel
            for record in message.authorities() {
                let ns_name = cast_or!(record.rdata(), RData::NS, continue);
                let records = self
                    .resolve_raw((ns_name, &RType::A, record.header().class()))
                    .await?;

                if !records.answers.is_empty() {
                    target_candidates.clear();

                    for answer in records.answers {
                        let ip_addr = cast_or!(answer.rdata(), RData::A, continue);
                        target_candidates.push(IpAddr::V4(*ip_addr))
                    }

                    break;
                }
            }
        }
    }
}

impl RecursiveResolver {
    pub async fn new(hint_file_path: String) -> Result<Self, ResolverError> {
        let client = match Client::new().await {
            Ok(client) => client,
            Err(_) => todo!(),
        };

        let zone = Zone::from_file(hint_file_path.into())?;
        let hints = zone.into_hints();

        let resolver = Self {
            hint_index: Mutex::new(0),
            client,
            hints,
        };

        Ok(resolver)
    }

    pub async fn find_glue_records(&self, message: &Message) -> Option<Vec<IpAddr>> {
        let mut ip_addrs: Vec<IpAddr> = Vec::new();

        // We look at each NS record and try to find a matching A record in the
        // additional record section.
        for ns_record in message.authorities() {
            let ns_name = cast_or!(ns_record.rdata(), RData::NS, continue);

            for ar_record in message.additionals() {
                if ar_record.header().name() != ns_name {
                    continue;
                }

                // We are only interested in A and AAAA records.
                match ar_record.rdata() {
                    RData::A(ip) => ip_addrs.push(IpAddr::V4(*ip)),
                    // RData::AAAA(ip) => ip_addrs.push(IpAddr::V6(*ip)),
                    _ => continue,
                }
            }
        }

        if !ip_addrs.is_empty() {
            return Some(ip_addrs);
        }

        None
    }

    pub fn hint(&self) -> IpAddr {
        // TODO (Techassi): Handle the unwrapping
        let mut hint_index = self.hint_index.lock().unwrap();

        if *hint_index >= self.hints.len() {
            *hint_index = 0;
        }

        let hint = self.hints.get(*hint_index).unwrap();
        *hint_index += 1;

        IpAddr::V4(hint.ipv4_addr)
    }
}
