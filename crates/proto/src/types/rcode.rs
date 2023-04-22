use std::fmt::Display;

/// [`Rcode`] describes the kind of response.
///
/// ### Notes
///
/// Response code - this 4 bit field is set as part of responses.
/// See [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1).
#[derive(Debug, Clone, Copy)]
pub enum Rcode {
    /// (0) No error condition.
    NoError,

    /// (1) Format error - The name server was unable to interpret the query.
    FormatError,

    /// (2) Server failure - The name server was unable to process this query due
    /// to a problem with the name server.
    ServerFailure,

    /// (3) Name Error - Meaningful only for responses from an authoritative name
    /// server, this code signifies that the domain name referenced in the query
    /// does not exist.
    NameError,

    /// (4) Not Implemented - The name server does not support the requested kind
    /// of query.
    NotImpl,

    /// (5) Refused - The name server refuses to perform the specified operation
    /// for policy reasons. For example, a name server may not wish to provide
    /// the information to the particular requester, or a name server may not
    /// wish to perform a particular operation (e.g., zone transfer) for
    /// particular data.
    Refused,

    /// (6-15) Initially reserved for future use (in RFC 1035).
    ///
    /// ### Notes
    ///
    /// RCODES above 15 (EXTENDED-RCODE) need to be combined with the upper 8
    /// bits in the OPT record. See [`edns::Header`][h] for more information.
    ///
    /// ### Changes
    ///
    /// - [[RFC 2671](https://www.rfc-editor.org/rfc/rfc2671#section-7)]
    ///   introduces the following changes: RCODE space is extended from 4 bits
    ///   to 12 bits. This will allow IANA to assign more than the 16 distinct
    ///   RCODE values allowed in
    ///   [[RFC1035](https://datatracker.ietf.org/doc/html/rfc1035)].
    /// - [[RFC 2671](https://www.rfc-editor.org/rfc/rfc2671#section-7)]
    ///   assigns EDNS Extended RCODE "16" to "BADVERS".
    ///
    /// [h]: crate::types::edns::Header
    Reserved,
}

impl Display for Rcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rcode::NoError => write!(f, "NOERROR"),
            Rcode::FormatError => write!(f, "FORMATERROR"),
            Rcode::ServerFailure => write!(f, "SERVERFAILURE"),
            Rcode::NameError => write!(f, "NAMEERROR"),
            Rcode::NotImpl => write!(f, "NOTIMPLEMENTED"),
            Rcode::Refused => write!(f, "REFUSED"),
            Rcode::Reserved => write!(f, "RESERVED"),
        }
    }
}

impl From<u16> for Rcode {
    fn from(code: u16) -> Self {
        match code {
            0 => Self::NoError,
            1 => Self::FormatError,
            2 => Self::ServerFailure,
            3 => Self::NameError,
            4 => Self::NotImpl,
            5 => Self::Refused,
            _ => Self::Reserved,
        }
    }
}

impl From<Rcode> for u16 {
    fn from(value: Rcode) -> Self {
        match value {
            Rcode::NoError => 0,
            Rcode::FormatError => 1,
            Rcode::ServerFailure => 2,
            Rcode::NameError => 3,
            Rcode::NotImpl => 4,
            Rcode::Refused => 5,
            Rcode::Reserved => 65535,
        }
    }
}
