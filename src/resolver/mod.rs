use std::fmt;

use enum_dispatch::enum_dispatch;

use crate::types::{dns::Message, rr::Record};

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

pub struct ResultRecords {
    answer: Vec<Record>,
    authority: Vec<Record>,
    additional: Vec<Record>,
}

impl fmt::Display for ResultRecords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Resolve results: \n  AN: {}\n  NS: {} \n  AR: {}",
            self.answer
                .iter()
                .map(|r| format!("    {}\n", r.to_string()))
                .collect::<String>(),
            self.authority
                .iter()
                .map(|r| format!("    {}\n", r.to_string()))
                .collect::<String>(),
            self.additional
                .iter()
                .map(|r| format!("    {}\n", r.to_string()))
                .collect::<String>(),
        )
    }
}

#[enum_dispatch(Resolver)]
pub trait ToResolver {
    /// Resolve resolves a query of a DNS [`Message`] by looking up via the lookup function.
    fn resolve(&self, message: &Message) -> ResolveResult;
    fn resolve_raw(&self, name: String, class: u16, typ: u16) -> ResolveResult;
    fn lookup(&self, name: String, class: u16, typ: u16) -> ResolveResult;
    fn refresh(&self, name: String, class: u16, typ: u16);
}

#[enum_dispatch]
pub enum Resolver {
    Recursive(recursive::RecursiveResolver),
    Iterative(iterative::IterativeResolver),
    Forwarding(forwarding::ForwardingResolver),
}

impl Resolver {
    /// Create a new [`Resolver`] based on the provided [`ResolveMode`].
    pub async fn new_from(mode: ResolveMode) -> Result<Self, ResolverError> {
        match mode {
            ResolveMode::Recursive => {
                return match RecursiveResolver::new().await {
                    Ok(resolver) => Ok(resolver.into()),
                    Err(err) => Err(err),
                }
            }
            ResolveMode::Iterative => todo!(),
            ResolveMode::Forwarding => todo!(),
        }
    }
}
