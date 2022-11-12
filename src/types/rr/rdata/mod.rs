use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{
    error::ProtocolError,
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::{
        dns::Name,
        rr::{RHeader, Type},
    },
};

mod hinfo;
mod minfo;
mod mx;
mod null;
mod opt;
mod soa;
mod txt;

use hinfo::*;
use minfo::*;
use mx::*;
use null::*;
use opt::*;
use soa::*;
use txt::*;

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

    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                     MNAME                     /
    /// /                                               /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                     RNAME                     /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    SERIAL                     |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    REFRESH                    |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                     RETRY                     |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    EXPIRE                     |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// |                    MINIMUM                    |
    /// |                                               |
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// where:
    ///
    /// MNAME           The <domain-name> of the name server that was the
    ///                 original or primary source of data for this zone.
    ///
    /// RNAME           A <domain-name> which specifies the mailbox of the
    ///                 person responsible for this zone.
    ///
    /// SERIAL          The unsigned 32 bit version number of the original
    ///                 copy of the zone.  Zone transfers preserve this value.
    ///                 This value wraps and should be compared using sequence
    ///                 space arithmetic.
    ///
    /// REFRESH         A 32 bit time interval before the zone should be
    ///                 refreshed.
    ///
    /// RETRY           A 32 bit time interval that should elapse before a
    ///                 failed refresh should be retried.
    ///
    /// EXPIRE          A 32 bit time value that specifies the upper limit on
    ///                 the time interval that can elapse before the zone is no
    ///                 longer authoritative.
    ///
    /// MINIMUM         The unsigned 32 bit minimum TTL field that should be
    /// exported with any RR from this zone.
    ///
    /// SOA records cause no additional section processing.
    ///
    /// All times are in units of seconds.
    ///
    /// Most of these fields are pertinent only for name server maintenance
    /// operations.  However, MINIMUM is used in all query operations that
    /// retrieve RRs from a zone.  Whenever a RR is sent in a response to a
    /// query, the TTL field is set to the maximum of the TTL field from the RR
    /// and the MINIMUM field in the appropriate SOA.  Thus MINIMUM is a lower
    /// bound on the TTL field for all RRs in a zone.  Note that this use of
    /// MINIMUM should occur when the RRs are copied into the response and not
    /// when the zone is loaded from a master file or via a zone transfer.  The
    /// reason for this provison is to allow future dynamic update facilities to
    /// change the SOA RR with known semantics.
    /// ```
    SOA(SOA),

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

    /// ```text
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    /// /                   TXT-DATA                    /
    /// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    ///
    /// where:
    ///
    /// TXT-DATA        One or more <character-string>s.
    ///
    /// TXT RRs are used to hold descriptive text. The semantics of the text
    /// depends on the domain where it is found.
    /// ```
    TXT(TXT),

    /// The AAAA resource record type is a record specific to the Internet
    /// class that stores a single IPv6 address.
    ///
    /// A 128 bit IPv6 address is encoded in the data portion of an AAAA
    /// resource record in network byte order (high-order byte first).
    AAAA(Ipv6Addr),
    OPT(OPT),
    AXFR,
    MAILB,
    MAILA,
    ANY,
    BOGUS,
}

impl Default for RData {
    fn default() -> Self {
        Self::NULL(NULL::new())
    }
}

impl ToString for RData {
    fn to_string(&self) -> String {
        match self {
            RData::A(a) => a.to_string(),
            RData::NS(ns) => ns.to_string(),
            RData::CNAME(cname) => cname.to_string(),
            RData::SOA(soa) => soa.to_string(),
            RData::NULL(_) => todo!(),
            RData::PTR(_) => todo!(),
            RData::HINFO(_) => todo!(),
            RData::MINFO(_) => todo!(),
            RData::MX(_) => todo!(),
            RData::TXT(_) => todo!(),
            RData::AAAA(aaaa) => aaaa.to_string(),
            RData::OPT(_) => todo!(),
            RData::AXFR => todo!(),
            RData::MAILB => todo!(),
            RData::MAILA => todo!(),
            RData::ANY => todo!(),
            RData::BOGUS => todo!(),
        }
    }
}

impl Packable for RData {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        match self {
            RData::A(a) => a.pack(buf),
            RData::NS(ns) => ns.pack(buf),
            RData::CNAME(cname) => cname.pack(buf),
            RData::SOA(soa) => soa.pack(buf),
            RData::NULL(_) => todo!(),
            RData::PTR(ptr) => ptr.pack(buf),
            RData::HINFO(_) => todo!(),
            RData::MINFO(_) => todo!(),
            RData::MX(_) => todo!(),
            RData::TXT(_) => todo!(),
            RData::AAAA(aaaa) => aaaa.pack(buf),
            RData::OPT(opt) => opt.pack(buf),
            RData::AXFR => todo!(),
            RData::MAILB => todo!(),
            RData::MAILA => todo!(),
            RData::ANY => todo!(),
            RData::BOGUS => todo!(),
        }
    }
}

impl RData {
    pub fn unpack(buf: &mut UnpackBuffer, header: &RHeader) -> UnpackBufferResult<Self> {
        let buf_offset_start = buf.offset();

        let result = match header.ty {
            Type::A => Ipv4Addr::unpack(buf).map(Self::A),
            Type::NS => Name::unpack(buf).map(Self::NS),
            Type::CNAME => Name::unpack(buf).map(Self::CNAME),
            Type::SOA => SOA::unpack(buf).map(Self::SOA),
            Type::NULL => NULL::unpack(buf, header.rdlen).map(Self::NULL),
            Type::PTR => Name::unpack(buf).map(Self::PTR),
            Type::HINFO => HINFO::unpack(buf).map(Self::HINFO),
            Type::MINFO => MINFO::unpack(buf).map(Self::MINFO),
            Type::MX => MX::unpack(buf).map(Self::MX),
            Type::TXT => TXT::unpack(buf, header.rdlen).map(Self::TXT),
            Type::AAAA => Ipv6Addr::unpack(buf).map(Self::AAAA),
            Type::OPT => OPT::unpack(buf, header).map(Self::OPT),
            Type::AXFR => todo!(),
            Type::MAILB => todo!(),
            Type::MAILA => todo!(),
            Type::ANY => todo!(),
            Type::UNKNOWN(_) => todo!(),
        };

        let rdata = match result {
            Ok(rdata) => rdata,
            Err(err) => return Err(err),
        };

        // Check that we read the correct number of octets defined by RDLEN
        let length_read = (buf.offset() - buf_offset_start) as u16;
        if length_read != header.rdlen {
            return Err(ProtocolError::InvalidRDataLenRead {
                expected: header.rdlen,
                found: length_read,
            });
        }

        Ok(rdata)
    }
}
