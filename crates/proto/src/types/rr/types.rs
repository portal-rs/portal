use std::{error::Error, fmt::Display, str::FromStr};

use binbuf::prelude::*;
use serde::Serialize;

#[derive(Debug)]
pub struct RTypeParseError(String);

impl Error for RTypeParseError {}

impl Display for RTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// [`Type`] describes resource record types.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.2
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum RType {
    /// A host address
    A,

    /// An authoritative name server
    NS,

    /// The canonical name for an alias
    CNAME,

    /// Marks the start of a zone of authority
    SOA,

    /// A null RR (EXPERIMENTAL)
    NULL,

    /// A domain name pointer
    PTR,

    // Host information
    HINFO,

    // Mailbox or mail list information
    MINFO,

    /// Mail exchange
    MX,

    /// Text strings
    TXT,

    /// AAAA host address
    AAAA,

    /// OPT Record / Meta record
    OPT,

    // QTypes are a superset of types and should only be allowed in questions
    /// A request for a transfer of an entire zone
    AXFR,

    /// A request for mailbox-related records (MB, MG or MR)
    MAILB,

    /// A request for mail agent RRs (Obsolete - see MX)
    MAILA,

    /// A request for all records (*)
    ANY,

    /// If we receive an invalid RR type, we fallback to this "type" after which we can terminate the connection
    UNKNOWN(u16),
}

impl Default for RType {
    fn default() -> Self {
        Self::NULL
    }
}

impl Readable for RType {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let ty = u16::read::<E>(buf)?;
        Ok(Self::from(ty))
    }
}

impl Writeable for RType {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let ty: u16 = self.into();
        ty.write::<E>(buf)
    }
}

impl Display for RType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RType::A => write!(f, "A"),
            RType::NS => write!(f, "NS"),
            RType::CNAME => write!(f, "CNAME"),
            RType::SOA => write!(f, "SOA"),
            RType::NULL => write!(f, "NULL"),
            RType::PTR => write!(f, "PTR"),
            RType::HINFO => write!(f, "HINFO"),
            RType::MINFO => write!(f, "MINFO"),
            RType::MX => write!(f, "MX"),
            RType::TXT => write!(f, "TXT"),
            RType::AAAA => write!(f, "AAAA"),
            RType::OPT => write!(f, "OPT"),
            RType::AXFR => write!(f, "AXFR"),
            RType::MAILB => write!(f, "MAILB"),
            RType::MAILA => write!(f, "MAILA"),
            RType::ANY => write!(f, "ANY"),
            RType::UNKNOWN(c) => write!(f, "UNKNOWN({c})"),
        }
    }
}

impl FromStr for RType {
    type Err = RTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::A),
            "NS" => Ok(Self::NS),
            "CNAME" => Ok(Self::CNAME),
            "SOA" => Ok(Self::SOA),
            "NULL" => Ok(Self::NULL),
            "PTR" => Ok(Self::PTR),
            "HINFO" => Ok(Self::HINFO),
            "MINFO" => Ok(Self::MINFO),
            "MX" => Ok(Self::MX),
            "TXT" => Ok(Self::TXT),
            "AAAA" => Ok(Self::AAAA),
            "OPT" => Ok(Self::OPT),
            "AXFR" => Ok(Self::AXFR),
            "MAILB" => Ok(Self::MAILB),
            "MAILA" => Ok(Self::MAILA),
            "ANY" => Ok(Self::ANY),
            _ => Err(RTypeParseError(format!("Invalid type: {s}"))),
        }
    }
}

impl TryFrom<&str> for RType {
    // TODO (Techassi): Change this to TypeParseError
    type Error = RTypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for RType {
    type Error = RTypeParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl From<u16> for RType {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            5 => Self::CNAME,
            6 => Self::SOA,
            10 => Self::NULL,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            28 => Self::AAAA,
            41 => Self::OPT,
            252 => Self::AXFR,
            253 => Self::MAILB,
            254 => Self::MAILA,
            255 => Self::ANY,
            _ => Self::UNKNOWN(value),
        }
    }
}

impl From<RType> for u16 {
    fn from(value: RType) -> Self {
        match value {
            RType::A => 1,
            RType::NS => 2,
            RType::CNAME => 5,
            RType::SOA => 6,
            RType::NULL => 10,
            RType::PTR => 12,
            RType::HINFO => 13,
            RType::MINFO => 14,
            RType::MX => 15,
            RType::TXT => 16,
            RType::AAAA => 28,
            RType::OPT => 41,
            RType::AXFR => 252,
            RType::MAILB => 253,
            RType::MAILA => 254,
            RType::ANY => 255,
            RType::UNKNOWN(v) => v,
        }
    }
}

impl From<&RType> for u16 {
    fn from(value: &RType) -> Self {
        Self::from(*value)
    }
}
