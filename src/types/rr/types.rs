use crate::packing::{
    PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
};

/// [`Type`] describes resource record types.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.2
#[derive(Debug, Clone, Copy)]
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

impl Unpackable for Type {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let ty = u16::unpack(buf)?;
        Ok(Self::from(ty))
    }
}

impl Packable for Type {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let ty: u16 = self.into();
        ty.pack(buf)
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::A => String::from("A"),
            Type::NS => String::from("NS"),
            Type::CNAME => String::from("CNAME"),
            Type::SOA => String::from("SOA"),
            Type::NULL => String::from("NULL"),
            Type::PTR => String::from("PTR"),
            Type::HINFO => String::from("HINFO"),
            Type::MINFO => String::from("MINFO"),
            Type::MX => String::from("MX"),
            Type::TXT => String::from("TXT"),
            Type::AAAA => String::from("AAAA"),
            Type::OPT => String::from("OPT"),
            Type::AXFR => String::from("AXFR"),
            Type::MAILB => String::from("MAILB"),
            Type::MAILA => String::from("MAILA"),
            Type::ANY => String::from("ANY"),
            Type::UNKNOWN(v) => format!("UNKNOWN({})", v),
        }
    }
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        return match value {
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
        };
    }
}

impl Into<u16> for Type {
    fn into(self) -> u16 {
        match self {
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

impl Into<u16> for &Type {
    fn into(self) -> u16 {
        (*self).into()
    }
}
