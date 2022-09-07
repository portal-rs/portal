use enum_dispatch::enum_dispatch;

use crate::packing::UnpackError;

mod classes;
mod records;
mod types;

pub use classes::*;
pub use records::*;
pub use types::*;

#[enum_dispatch(ResourceRecord)]
pub trait Record: ToString {
    /// Retrieve the [`RRHeader`].
    fn header(&self) -> &RRHeader;

    fn set_header(&mut self, header: RRHeader);
    fn len(&self) -> u16;
    fn unpack(&self, data: &Vec<u8>, offset: usize) -> Result<usize, UnpackError>;
    fn pack(&self, buf: &Vec<u8>, offset: usize) -> Result<usize, ()>;
}

#[derive(Debug)]
#[enum_dispatch]
pub enum ResourceRecord {
    A(A),
    AAAA(AAAA),
    CNAME(CNAME),
    HINFO(HINFO),
    MB(MB),
    MD(MD),
    MF(MF),
    MG(MG),
    MINFO(MINFO),
    MR(MR),
    MX(MX),
    NS(NS),
    NULL(NULL),
    // OPT(OPT),
    PTR(PTR),
    // SOA(SOA),
    TXT(TXT),
}

impl ToString for ResourceRecord {
    fn to_string(&self) -> String {
        match self {
            Self::A(r) => r.to_string(),
            Self::AAAA(r) => r.to_string(),
            Self::CNAME(r) => r.to_string(),
            Self::HINFO(r) => r.to_string(),
            Self::MB(r) => r.to_string(),
            Self::MD(r) => r.to_string(),
            Self::MF(r) => r.to_string(),
            Self::MG(r) => r.to_string(),
            Self::MINFO(r) => r.to_string(),
            Self::MR(r) => r.to_string(),
            Self::MX(r) => r.to_string(),
            Self::NS(r) => r.to_string(),
            Self::NULL(r) => r.to_string(),
            Self::PTR(r) => r.to_string(),
            Self::TXT(r) => r.to_string(),
        }
    }
}

impl From<RRHeader> for ResourceRecord {
    fn from(header: RRHeader) -> Self {
        match header.typ {
            Type::NONE => todo!(),
            Type::A => A::new_with_header(header),
            Type::NS => NS::new_with_header(header),
            Type::MD => MD::new_with_header(header),
            Type::MF => MF::new_with_header(header),
            Type::CNAME => CNAME::new_with_header(header),
            Type::SOA => todo!(),
            Type::MB => MB::new_with_header(header),
            Type::MG => MG::new_with_header(header),
            Type::MR => MR::new_with_header(header),
            Type::NULL => NULL::new_with_header(header),
            Type::WKS => todo!(),
            Type::PTR => PTR::new_with_header(header),
            Type::HINFO => HINFO::new_with_header(header),
            Type::MINFO => MINFO::new_with_header(header),
            Type::MX => MX::new_with_header(header),
            Type::TXT => TXT::new_with_header(header),
            Type::AAAA => AAAA::new_with_header(header),
            Type::OPT => todo!(),
            Type::AXFR => todo!(),
            Type::MAILB => todo!(),
            Type::MAILA => todo!(),
            Type::ANY => todo!(),
            Type::BOGUS => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct RRHeader {
    pub name: String,
    pub typ: Type,
    pub class: Class,
    pub ttl: u32,
    pub rdlen: u16,
}
