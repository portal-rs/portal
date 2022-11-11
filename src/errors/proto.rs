use std::fmt::Display;

use crate::constants;

pub enum ProtocolError {
    InvalidRDataLenRead { expected: u16, found: u16 },
    InvalidLabelLenOrPointer(u8),
    InvalidPointerLocation,
    DomainNameLabelTooLong,
    UnpackFailure(String),
    PackFailure(String),
    DomainNameTooLong,
    BufTooShort,
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::InvalidRDataLenRead { expected, found } => write!(
                f,
                "Invalid RDATA length read (Expected: {}, Found: {})",
                expected, found
            ),
            ProtocolError::InvalidLabelLenOrPointer(ptr) => write!(
                f,
                "Invalid domain name label length or compression pointer ({})",
                ptr
            ),
            ProtocolError::UnpackFailure(details) => write!(f, "Failed to unpack: {}", details),
            ProtocolError::PackFailure(details) => write!(f, "Failed to pack: {}", details),
            ProtocolError::InvalidPointerLocation => {
                write!(f, "Invalid compression pointer location")
            }
            ProtocolError::DomainNameLabelTooLong => write!(
                f,
                "Domain name label too long (< {})",
                constants::dns::MAX_LABEL_LENGTH
            ),
            ProtocolError::DomainNameTooLong => write!(
                f,
                "Domain name to long (< {})",
                constants::dns::MAX_DOMAIN_LENGTH
            ),
            ProtocolError::BufTooShort => write!(f, "Buf too short"),
        }
    }
}
