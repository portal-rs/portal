use enum_dispatch::enum_dispatch;

use crate::packing::UnpackError;

mod classes;
mod types;

pub use classes::*;
pub use types::*;

mod a;
mod aaaa;
mod cname;

pub use a::*;
pub use aaaa::*;
pub use cname::*;

#[enum_dispatch(ResourceRecord)]
pub trait Record: ToString + PartialEq<Self> {
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
}

impl ToString for ResourceRecord {
    fn to_string(&self) -> String {
        match self {
            Self::A(r) => r.to_string(),
            Self::AAAA(r) => r.to_string(),
            Self::CNAME(r) => r.to_string(),
        }
    }
}

impl PartialEq<Self> for ResourceRecord {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ResourceRecord::A(_), ResourceRecord::A(_)) => todo!(),
            (ResourceRecord::A(_), ResourceRecord::AAAA(_)) => todo!(),
            (ResourceRecord::A(_), ResourceRecord::CNAME(_)) => todo!(),
            (ResourceRecord::AAAA(_), ResourceRecord::A(_)) => todo!(),
            (ResourceRecord::AAAA(_), ResourceRecord::AAAA(_)) => todo!(),
            (ResourceRecord::AAAA(_), ResourceRecord::CNAME(_)) => todo!(),
            (ResourceRecord::CNAME(_), ResourceRecord::A(_)) => todo!(),
            (ResourceRecord::CNAME(_), ResourceRecord::AAAA(_)) => todo!(),
            (ResourceRecord::CNAME(_), ResourceRecord::CNAME(_)) => todo!(),
        }
    }
}

impl From<RRHeader> for ResourceRecord {
    fn from(header: RRHeader) -> Self {
        match header.typ {
            Type::NONE => todo!(),
            Type::A => A::new_with_header(header),
            Type::NS => todo!(),
            Type::MD => todo!(),
            Type::MF => todo!(),
            Type::CNAME => CNAME::new_with_header(header),
            Type::SOA => todo!(),
            Type::MB => todo!(),
            Type::MG => todo!(),
            Type::MR => todo!(),
            Type::NULL => todo!(),
            Type::WKS => todo!(),
            Type::PTR => todo!(),
            Type::HINFO => todo!(),
            Type::MINFO => todo!(),
            Type::MX => todo!(),
            Type::TXT => todo!(),
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
