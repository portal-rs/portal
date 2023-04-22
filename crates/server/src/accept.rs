use portal_proto::{Header, Opcode};

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

    // If there is no question section at all
    if header.qdcount == 0 {
        return Action::Ignore;
    }

    if header.opcode != Opcode::Query {
        return Action::NoImpl;
    }

    // If there is more than one question, we reject. Most DNS Servers and
    // resolvers don't implement this feature.
    if header.qdcount > 1 {
        return Action::Reject;
    }

    Action::Accept
}
