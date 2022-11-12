use thiserror::Error;

use crate::constants;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Invalid RDATA length read (Expected: {expected}, Found: {found})")]
    InvalidRDataLenRead { expected: u16, found: u16 },

    #[error("Invalid domain name label length or compression pointer ({0})")]
    InvalidLabelLenOrPointer(u8),

    #[error("Invalid compression pointer location")]
    InvalidPointerLocation,

    #[error("Domain name label too long (< {})", constants::dns::MAX_LABEL_LENGTH)]
    DomainNameLabelTooLong,

    #[error("Failed to unpack ({0})")]
    UnpackFailure(String),

    #[error("Failed to pack ({0})")]
    PackFailure(String),

    #[error("Domain name to long (< {})", constants::dns::MAX_DOMAIN_LENGTH)]
    DomainNameTooLong,

    #[error("Buf too short")]
    BufTooShort,
}
