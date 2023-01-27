/// [`Opcode`] describes the kind of query of the message.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Query,
    IQuery,
    Status,
    Reserved,
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

impl Into<u16> for Opcode {
    fn into(self) -> u16 {
        match self {
            Opcode::Query => 0,
            Opcode::IQuery => 1,
            Opcode::Status => 2,
            Opcode::Reserved => 65535,
        }
    }
}
