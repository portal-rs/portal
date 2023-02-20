use std::fmt::Display;

/// [`Opcode`] describes the kind of query of the message.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Query,
    IQuery,
    Status,
    Reserved,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Query => write!(f, "QUERY"),
            Opcode::IQuery => write!(f, "IQUERY"),
            Opcode::Status => write!(f, "STATUS"),
            Opcode::Reserved => write!(f, "RESERVED"),
        }
    }
}

impl From<u16> for Opcode {
    fn from(code: u16) -> Self {
        match code {
            0 => Self::Query,
            1 => Self::IQuery,
            2 => Self::Status,
            _ => Self::Reserved,
        }
    }
}

impl From<Opcode> for u16 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Query => 0,
            Opcode::IQuery => 1,
            Opcode::Status => 2,
            Opcode::Reserved => 65535,
        }
    }
}
