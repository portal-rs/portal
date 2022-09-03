/// [`Rcode`] describes the kind of response.
pub enum Rcode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImpl,
    Refused,
    Reserved,
}

impl From<u16> for Rcode {
    fn from(code: u16) -> Self {
        return match code {
            0 => Self::NoError,
            1 => Self::FormatError,
            2 => Self::ServerFailure,
            3 => Self::NameError,
            4 => Self::NotImpl,
            5 => Self::Refused,
            _ => Self::Reserved,
        };
    }
}
