use std::fmt::Display;

pub enum UnpackError {
    InvalidLabelLenOrPointer(u8),
    InvalidPointerLocation,
    DomainNameLabelTooLong,
    DomainNameTooLong,
    MissingHeader,
    NoMessageBody,
    TooShort,
}

impl Display for UnpackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnpackError::InvalidLabelLenOrPointer(b) => {
                write!(f, "Invalid domain name label ({})", b)
            }
            UnpackError::InvalidPointerLocation => {
                write!(f, "Invalid compression pointer location")
            }
            UnpackError::DomainNameLabelTooLong => write!(f, "Domain name label too long"),
            UnpackError::DomainNameTooLong => write!(f, "Domain name too long"),
            UnpackError::MissingHeader => write!(f, "Missing header"),
            UnpackError::NoMessageBody => write!(f, "No body data"),
            UnpackError::TooShort => write!(f, "Buf too short"),
        }
    }
}

pub enum PackError {
    TooShort,
}
