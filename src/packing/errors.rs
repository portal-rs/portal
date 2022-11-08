use std::fmt::Display;

pub enum PackingError {
    InvalidLabelLenOrPointer(u8),
    InvalidPointerLocation,
    DomainNameLabelTooLong,
    DomainNameTooLong,
    MissingHeader,
    NoMessageBody,
    TooShort,
}

impl Display for PackingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackingError::InvalidLabelLenOrPointer(b) => {
                write!(f, "Invalid domain name label ({})", b)
            }
            PackingError::InvalidPointerLocation => {
                write!(f, "Invalid compression pointer location")
            }
            PackingError::DomainNameLabelTooLong => write!(f, "Domain name label too long"),
            PackingError::DomainNameTooLong => write!(f, "Domain name too long"),
            PackingError::MissingHeader => write!(f, "Missing header"),
            PackingError::NoMessageBody => write!(f, "No body data"),
            PackingError::TooShort => write!(f, "Buf too short"),
        }
    }
}
