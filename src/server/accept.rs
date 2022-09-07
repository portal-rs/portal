use crate::types::{dns::Header, opcode::Opcode};

pub enum Action {
    Accept,
    Reject,
    Ignore,
    NoImpl,
}

/// This function decides if the server should accept the incoming DNS message
/// based on the already unpacked DNS [`Header`].
pub async fn should_accept(header: &Header) -> Action {
    if !header.is_query {
        return Action::Ignore;
    }

    if !matches!(header.opcode, Opcode::Query) {
        return Action::NoImpl;
    }

    // If there is more than one question, we reject. Most DNS Servers and
    // resolvers don't implement this feature.
    if header.qdcount != 1 {
        return Action::Reject;
    }

    return Action::Accept;
}
