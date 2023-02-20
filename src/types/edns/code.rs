use binbuf::prelude::*;

/// DNS EDNS0 Option Codes (OPT)
///
/// ### See
/// - https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-11
/// - https://datatracker.ietf.org/doc/html/rfc6891
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OptionCode {
    /// 0, 4 and 65535 are reserved
    RESERVED(u16),

    /// 65001-65534 reserved for local/experimental use
    RESERVEDLOCAL(u16),

    /// Unassigned
    /// - 18 - 20291
    /// - 20293 - 26945
    /// - 26947 - 65000
    UNASSIGNED,

    /// Long-Lived Queries
    /// [[RFC 8764](https://datatracker.ietf.org/doc/html/rfc8764)]
    LLQ,

    /// Update Leases (Draft)
    UL,

    /// Name Server Identifier
    /// [[RFC 5001](https://datatracker.ietf.org/doc/html/rfc5001)]
    NSID,

    /// DNSSEC Algorithm Understood (DAU)
    /// [[RFC 6975](https://datatracker.ietf.org/doc/html/rfc6975#section-3)]
    DAU,

    /// DS Hash Understood (DHU)
    /// [[RFC 6975](https://datatracker.ietf.org/doc/html/rfc6975#section-3)]
    DHU,

    /// NSEC3 Hash Understood (N3U)
    /// [[RFC 6975](https://datatracker.ietf.org/doc/html/rfc6975#section-3)]
    N3U,

    /// Client Subnet in DNS Queries
    /// [[RFC 7871](https://datatracker.ietf.org/doc/html/rfc7871)]
    ECS,
    EXPIRE,

    /// Domain Name System (DNS) Cookies
    /// [[RFC 7873](https://datatracker.ietf.org/doc/html/rfc7873)]
    COOKIE,

    /// The edns-tcp-keepalive EDNS0 Option
    /// [[RFC 7828](https://datatracker.ietf.org/doc/html/rfc7828)]
    TCPKEEPALIVE,

    /// The EDNS(0) Padding Option
    /// [[RFC 7830](https://datatracker.ietf.org/doc/html/rfc7830)]
    PADDING,

    ///
    CHAIN,
    KEYTAG,
    EDE,
    CLIENTTAG,
    SERVERTAG,
    UMBRELLAIDENT,
    DEVICEID,
}

impl Readable for OptionCode {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let code = u16::read::<E>(buf)?;
        Ok(OptionCode::from(code))
    }
}

impl Writeable for OptionCode {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let code: u16 = self.into();
        code.write::<E>(buf)
    }
}

impl From<u16> for OptionCode {
    fn from(value: u16) -> Self {
        match value {
            1 => OptionCode::LLQ,
            2 => OptionCode::UL,
            3 => OptionCode::NSID,
            5 => OptionCode::DAU,
            6 => OptionCode::DHU,
            7 => OptionCode::N3U,
            8 => OptionCode::ECS,
            9 => OptionCode::EXPIRE,
            10 => OptionCode::COOKIE,
            11 => OptionCode::TCPKEEPALIVE,
            12 => OptionCode::PADDING,
            13 => OptionCode::CHAIN,
            14 => OptionCode::KEYTAG,
            15 => OptionCode::EDE,
            16 => OptionCode::CLIENTTAG,
            17 => OptionCode::SERVERTAG,
            20292 => OptionCode::UMBRELLAIDENT,
            26946 => OptionCode::DEVICEID,
            0 | 4 | u16::MAX => OptionCode::RESERVED(value),
            65001..=65534 => OptionCode::RESERVEDLOCAL(value),
            _ => Self::UNASSIGNED,
        }
    }
}

impl From<OptionCode> for u16 {
    fn from(value: OptionCode) -> Self {
        match value {
            OptionCode::RESERVED(_) => todo!(),
            OptionCode::RESERVEDLOCAL(_) => todo!(),
            OptionCode::UNASSIGNED => todo!(),
            OptionCode::LLQ => todo!(),
            OptionCode::UL => todo!(),
            OptionCode::NSID => todo!(),
            OptionCode::DAU => todo!(),
            OptionCode::DHU => todo!(),
            OptionCode::N3U => todo!(),
            OptionCode::ECS => todo!(),
            OptionCode::EXPIRE => todo!(),
            OptionCode::COOKIE => 10,
            OptionCode::TCPKEEPALIVE => todo!(),
            OptionCode::PADDING => todo!(),
            OptionCode::CHAIN => todo!(),
            OptionCode::KEYTAG => todo!(),
            OptionCode::EDE => todo!(),
            OptionCode::CLIENTTAG => todo!(),
            OptionCode::SERVERTAG => todo!(),
            OptionCode::UMBRELLAIDENT => todo!(),
            OptionCode::DEVICEID => todo!(),
        }
    }
}

impl From<&OptionCode> for u16 {
    fn from(value: &OptionCode) -> Self {
        Self::from(*value)
    }
}
