/// [`Opcode`] describes the kind of query of the message.
pub enum Opcode {
    Query,
    IQuery,
    Status,
    Reserved,
}

impl From<u16> for Opcode {
    fn from(code: u16) -> Self {
        return match code {
            0 => Self::Query,
            1 => Self::IQuery,
            2 => Self::Status,
            _ => Self::Reserved,
        };
    }
}
