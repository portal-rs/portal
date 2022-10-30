use std::net::Ipv4Addr;

use crate::{
    packing::{UnpackBuffer, UnpackBufferResult, Unpackable},
    types::{
        dns::Name,
        rr::{RHeader, Type},
    },
};

mod hinfo;
mod minfo;
mod mx;
mod null;

use hinfo::*;
use minfo::*;
use mx::*;
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
    // TODO (Techassi): Implement SOA
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

    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                   PTRDNAME                    /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// where:
    ///
    /// PTRDNAME        A <domain-name> which points to some location in the
    ///                 domain name space.
    ///
    /// PTR records cause no additional section processing.  These RRs are used
    /// in special domains to point to some other location in the domain space.
    /// These records are simple data, and don't imply any special processing
    /// similar to that performed by CNAME, which identifies aliases.  See the
    /// description of the IN-ADDR.ARPA domain for an example.
    /// ```
    PTR(Name),

    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                      CPU                      /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                       OS                      /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// where:
    ///
    /// CPU             A <character-string> which specifies the CPU type.
    ///
    /// OS              A <character-string> which specifies the operating
    ///                 system type.
    ///
    /// Standard values for CPU and OS can be found in [RFC-1010].
    ///
    /// HINFO records are used to acquire general information about a host. The
    /// main use is for protocols such as FTP that can use special procedures
    /// when talking between machines or operating systems of the same type.
    /// ```
    HINFO(HINFO),

    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                    RMAILBX                    /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                    EMAILBX                    /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// where:
    ///
    /// RMAILBX         A <domain-name> which specifies a mailbox which is
    ///                 responsible for the mailing list or mailbox. If this
    ///                 domain name names the root, the owner of the MINFO RR is
    ///                 responsible for itself. Note that many existing mailing
    ///                 lists use a mailbox X-request for the RMAILBX field of
    ///                 mailing list X, e.g., Msgroup-request for Msgroup. This
    ///                 field provides a more general mechanism.
    ///
    ///
    /// EMAILBX         A <domain-name> which specifies a mailbox which is to
    ///                 receive error messages related to the mailing list or
    ///                 mailbox specified by the owner of the MINFO RR (similar
    ///                 to the ERRORS-TO: field which has been proposed). If
    ///                 this domain name names the root, errors should be
    ///                 returned to the sender of the message.
    ///
    /// MINFO records cause no additional section processing. Although these
    /// records can be associated with a simple mailbox, they are usually used
    /// with a mailing list.
    /// ```
    MINFO(MINFO),

    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                  PREFERENCE                   |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                   EXCHANGE                    /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// where:
    ///
    /// PREFERENCE      A 16 bit integer which specifies the preference given
    ///                 to this RR among others at the same owner. Lower values
    ///                 are preferred.
    ///
    /// EXCHANGE        A <domain-name> which specifies a host willing to act
    ///                 as a mail exchange for the owner name.
    ///
    /// MX records cause type A additional section processing for the host
    /// specified by EXCHANGE.  The use of MX RRs is explained in detail in
    /// [RFC-974].
    /// ```
    MX(MX),
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
            Type::PTR => Name::unpack(buf).map(Self::PTR),
            Type::HINFO => HINFO::unpack(buf).map(Self::HINFO),
            Type::MINFO => MINFO::unpack(buf).map(Self::MINFO),
            Type::MX => MX::unpack(buf).map(Self::MX),
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
