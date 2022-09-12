/// [`Type`] describes resource record types.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.2
#[derive(Debug, Clone)]
pub enum Type {
    NONE,
    /// A host address
    A,

    /// An authoritative name server
    NS,

    /// A mail destination (Obsolete - use MX)
    MD,

    /// A mail forwarder (Obsolete - use MX)
    MF,

    /// The canonical name for an alias
    CNAME,

    /// Marks the start of a zone of authority
    SOA,

    /// A mailbox domain name (EXPERIMENTAL)
    MB,

    /// A mail group member (EXPERIMENTAL)
    MG,

    /// A mail rename domain name (EXPERIMENTAL)
    MR,

    /// A null RR (EXPERIMENTAL)
    NULL,

    /// A well known service description
    WKS,

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
    BOGUS,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::NONE => String::from("NONE"),
            Type::A => String::from("A"),
            Type::NS => String::from("NS"),
            Type::MD => String::from("MD"),
            Type::MF => String::from("MF"),
            Type::CNAME => String::from("CNAME"),
            Type::SOA => String::from("SOA"),
            Type::MB => String::from("MB"),
            Type::MG => String::from("MG"),
            Type::MR => String::from("MR"),
            Type::NULL => String::from("NULL"),
            Type::WKS => String::from("WKS"),
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
            Type::BOGUS => String::from("BOGUS"),
        }
    }
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        return match value {
            0 => Self::NONE,
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::NULL,
            11 => Self::WKS,
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
            _ => Self::BOGUS,
        };
    }
}
