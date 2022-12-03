use std::fmt::Display;

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::types::{
    dns::{Message, ToQuery},
    rr::Record,
};

mod error;
mod forwarding;
mod iterative;
mod mode;
mod recursive;

pub use error::*;
pub use forwarding::*;
pub use iterative::*;
pub use mode::*;
pub use recursive::*;

pub type ResolveResult = Result<ResultRecords, ResolverError>;

#[derive(Debug)]
pub struct ResultRecords {
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub additionals: Vec<Record>,
}

impl From<Message> for ResultRecords {
    fn from(msg: Message) -> Self {
        Self {
            answers: msg.answers,
            authorities: msg.authorities,
            additionals: msg.additionals,
        }
    }
}

impl Display for ResultRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Resolve results: \n  AN: {}\n  NS: {} \n  AR: {}",
            self.answers
                .iter()
                .map(|r| format!("    {}\n", r.to_string()))
                .collect::<String>(),
            self.authorities
                .iter()
                .map(|r| format!("    {}\n", r.to_string()))
                .collect::<String>(),
            self.additionals
                .iter()
                .map(|r| format!("    {}\n", r.to_string()))
                .collect::<String>(),
        )
    }
}

impl ResultRecords {
    pub fn normalize_rdlens(&mut self) {
        for answer in &mut self.answers {
            answer.normalize_rdlen();
        }
        for authority in &mut self.authorities {
            authority.normalize_rdlen();
        }
        for additional in &mut self.additionals {
            additional.normalize_rdlen();
        }
    }
}

#[async_trait]
#[enum_dispatch(Resolver)]
pub trait ToResolver {
    /// Resolve resolves a query of a DNS [`Message`] by looking up via the lookup function.
    async fn resolve(&self, message: &Message) -> ResolveResult;
    async fn resolve_raw<Q: ToQuery>(&self, query: Q) -> ResolveResult;
    // async fn lookup<Q: ToQuery>(&self, query: Q) -> ResolveResult;
    // async fn refresh<Q: ToQuery>(&self, query: Q);
}

#[enum_dispatch]
pub enum Resolver {
    Recursive(recursive::RecursiveResolver),
    Iterative(iterative::IterativeResolver),
    Forwarding(forwarding::ForwardingResolver),
}
