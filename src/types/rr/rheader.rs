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

impl Unpackable for RHeader {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let name = match Name::unpack(buf) {
            Ok(name) => name,
            Err(err) => return Err(err),
        };

        let ty = match Type::unpack(buf) {
            Ok(ty) => ty,
            Err(err) => return Err(err),
        };

        let class = match Class::unpack(buf) {
            Ok(class) => class,
            Err(err) => return Err(err),
        };

        let ttl = match u32::unpack(buf) {
            Ok(ttl) => ttl,
            Err(err) => return Err(err),
        };

        let rdlen = match u16::unpack(buf) {
            Ok(rdlen) => rdlen,
            Err(err) => return Err(err),
        };

        Ok(Self {
            name,
            ty,
            class,
            ttl,
            rdlen,
        })
    }
}
