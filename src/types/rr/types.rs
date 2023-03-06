use std::{error::Error, fmt::Display, str::FromStr};

use binbuf::prelude::*;

#[derive(Debug)]
pub struct TypeParseError(String);

impl Error for TypeParseError {}

impl Display for TypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// [`Type`] describes resource record types.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.2
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
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

impl Default for Type {
    fn default() -> Self {
        Self::NULL
    }
}

impl Readable for Type {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let ty = u16::read::<E>(buf)?;
        Ok(Self::from(ty))
    }
}

impl Writeable for Type {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let ty: u16 = self.into();
        ty.write::<E>(buf)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::A => write!(f, "A"),
            Type::NS => write!(f, "NS"),
            Type::CNAME => write!(f, "CNAME"),
            Type::SOA => write!(f, "SOA"),
            Type::NULL => write!(f, "NULL"),
            Type::PTR => write!(f, "PTR"),
            Type::HINFO => write!(f, "HINFO"),
            Type::MINFO => write!(f, "MINFO"),
            Type::MX => write!(f, "MX"),
            Type::TXT => write!(f, "TXT"),
            Type::AAAA => write!(f, "AAAA"),
            Type::OPT => write!(f, "OPT"),
            Type::AXFR => write!(f, "AXFR"),
            Type::MAILB => write!(f, "MAILB"),
            Type::MAILA => write!(f, "MAILA"),
            Type::ANY => write!(f, "ANY"),
            Type::UNKNOWN(c) => write!(f, "UNKNOWN({c})"),
        }
    }
}

impl FromStr for Type {
    type Err = TypeParseError;

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
            _ => Err(TypeParseError(format!("Invalid type: {s}"))),
        }
    }
}

impl TryFrom<&str> for Type {
    // TODO (Techassi): Change this to TypeParseError
    type Error = TypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Type {
    type Error = TypeParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl From<u16> for Type {
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

impl From<Type> for u16 {
    fn from(value: Type) -> Self {
        match value {
            Type::A => 1,
            Type::NS => 2,
            Type::CNAME => 5,
            Type::SOA => 6,
            Type::NULL => 10,
            Type::PTR => 12,
            Type::HINFO => 13,
            Type::MINFO => 14,
            Type::MX => 15,
            Type::TXT => 16,
            Type::AAAA => 28,
            Type::OPT => 41,
            Type::AXFR => 252,
            Type::MAILB => 253,
            Type::MAILA => 254,
            Type::ANY => 255,
            Type::UNKNOWN(v) => v,
        }
    }
}

impl From<&Type> for u16 {
    fn from(value: &Type) -> Self {
        Self::from(*value)
    }
}
