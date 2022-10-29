use std::net::Ipv4Addr;

use crate::{
    packing::{UnpackBuffer, UnpackBufferResult, Unpackable},
    types::{
        dns::Name,
        rr::{RHeader, Type},
    },
};

mod null;

use null::*;

#[derive(Debug)]
pub enum RData {
    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    ADDRESS                    |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// where:
    ///
    /// ADDRESS         A 32 bit Internet address.
    ///
    /// Hosts that have multiple Internet addresses will have multiple A records.
    /// A records cause no additional section processing.  The RDATA section of an
    /// A line in a master file is an Internet address expressed as four decimal
    /// numbers separated by dots without any imbedded spaces (e.g., "10.2.0.52" or
    /// "192.0.5.6").
    /// ```
    ///
    /// ### See
    ///
    /// - https://datatracker.ietf.org/doc/html/rfc1035
    /// - https://datatracker.ietf.org/doc/html/rfc1035#section-3.4.1
    /// - https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.3
    A(Ipv4Addr),

    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                   NSDNAME                     /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// where:
    ///
    /// NSDNAME     A <domain-name> which specifies a host which should be
    ///             authoritative for the specified class and domain.
    ///
    /// NS records cause both the usual additional section processing to locate
    /// a type A record, and, when used in a referral, a special search of the
    /// zone in which they reside for glue information.
    ///
    /// The NS RR states that the named host should be expected to have a zone
    /// starting at owner name of the specified class.  Note that the class may
    /// not indicate the protocol family which should be used to communicate
    /// with the host, although it is typically a strong hint.  For example,
    /// hosts which are name servers for either Internet (IN) or Hesiod (HS)
    /// class information are normally queried using IN class protocols.
    /// ```
    ///
    /// ### See
    ///
    /// - https://datatracker.ietf.org/doc/html/rfc1035
    /// - https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.11
    /// - https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.3
    NS(Name),

    ///```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                     CNAME                     /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    ///where:
    ///
    ///CNAME           A <domain-name> which specifies the canonical or primary
    ///                name for the owner. The owner name is an alias.
    ///
    ///CNAME RRs cause no additional section processing, but name servers may
    ///choose to restart the query at the canonical name in certain cases. See
    ///the description of name server logic in [RFC-1034] for details.
    /// ```
    CNAME(Name),
    SOA,

    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                  <anything>                   /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// Anything at all may be in the RDATA field so long as it is 65535 octets
    /// or less. NULL records cause no additional section processing. NULL RRs
    /// are not allowed in master files.  NULLs are used as placeholders in
    /// some experimental extensions of the DNS.
    /// ```
    NULL(NULL),
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
    AAAA,
    OPT,
    AXFR,
    MAILB,
    MAILA,
    ANY,
    BOGUS,
}

impl RData {
    pub fn unpack(buf: &mut UnpackBuffer, header: RHeader) -> UnpackBufferResult<Self> {
        let result = match header.ty {
            Type::A => Ipv4Addr::unpack(buf).map(Self::A),
            Type::NS => Name::unpack(buf).map(Self::NS),
            Type::CNAME => Name::unpack(buf).map(Self::CNAME),
            Type::SOA => todo!(),
            Type::NULL => NULL::unpack(buf, header.rdlen).map(Self::NULL),
            Type::WKS => todo!(),
            Type::PTR => todo!(),
            Type::HINFO => todo!(),
            Type::MINFO => todo!(),
            Type::MX => todo!(),
            Type::TXT => todo!(),
            Type::AAAA => todo!(),
            Type::OPT => todo!(),
            Type::AXFR => todo!(),
            Type::MAILB => todo!(),
            Type::MAILA => todo!(),
            Type::ANY => todo!(),
            Type::BOGUS => todo!(),
        };

        let rdata = match result {
            Ok(rdata) => rdata,
            Err(err) => return Err(err),
        };

        // Check header rdlen against real len

        Ok(rdata)
    }
}

impl Default for RData {
    fn default() -> Self {
        Self::NULL(NULL::new())
    }
}
