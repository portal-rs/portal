/// [`Class`] describes resource record class codes.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4
#[derive(Debug)]
pub enum Class {
    /// The Internet
    IN,

    /// The CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS,

    /// The CHAOS class
    CH,

    /// Hesiod [Dyer 87]
    HS,

    // QClasses are a superset of classes and should only be allowed in questions
    /// Any class (*)
    ANY,

    /// If we receive an invalid RR class, we fallback to this "class" after which we can terminate the connection
    BOGUS,
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        return match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            255 => Self::ANY,
            _ => Self::BOGUS,
        };
    }
}
