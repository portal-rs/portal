use std::net::AddrParseError;

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

    #[error("Invalid byte in domain name label")]
    InvalidDomainNameLabelByte,

    #[error("Domain name label too long (< {})", constants::dns::MAX_LABEL_LENGTH)]
    DomainNameLabelTooLong,

    #[error("Failed to unpack ({0})")]
    UnpackError(String),

    #[error("Failed to pack ({0})")]
    PackError(String),

    #[error("Domain name to long (< {})", constants::dns::MAX_DOMAIN_LENGTH)]
    DomainNameTooLong,

    #[error("Character string length exceeded the provided max length of {0}")]
    CharStringExceededMaxLen(u8),

    #[error("Failed to parse RR class from string")]
    ClassParseError,

    #[error("Failed to parse RR type from string")]
    TypeParseError,

    #[error("Buf too short")]
    BufTooShort,

    #[error("IP address parse error: {0}")]
    AddrParseError(#[from] AddrParseError),
}
