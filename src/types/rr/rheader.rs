use crate::{
    packing::{UnpackBuffer, UnpackBufferResult, Unpackable},
    types::{
        dns::Name,
        rr::{Class, Type},
    },
};

#[derive(Debug)]
pub struct RHeader {
    pub name: Name,
    pub ty: Type,
    pub class: Class,
    pub ttl: u32,
    pub rdlen: u16,
}

impl Default for RHeader {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ty: Default::default(),
            class: Default::default(),
            ttl: Default::default(),
            rdlen: Default::default(),
        }
    }
}

impl ToString for RHeader {
    fn to_string(&self) -> String {
        format!(
            "N: {} T: {} C: {} TTL: {} ({})",
            self.name.to_string(),
            self.ty.to_string(),
            self.class.to_string(),
            self.ttl,
            self.rdlen
        )
    }
}

impl Unpackable for RHeader {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let name = Name::unpack(buf)?;
        let ty = Type::unpack(buf)?;
        let class = Class::unpack(buf)?;
        let ttl = u32::unpack(buf)?;
        let rdlen = u16::unpack(buf)?;

        Ok(Self {
            name,
            ty,
            class,
            ttl,
            rdlen,
        })
    }
}
