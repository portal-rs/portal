use crate::{
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::{
        dns::Name,
        rr::{Class, Type},
    },
};

#[derive(Debug, Clone)]
pub struct RHeader {
    name: Name,
    ty: Type,
    class: Class,
    ttl: u32,
    rdlen: u16,
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

impl Packable for RHeader {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        self.name.pack(buf)?;
        self.ty.pack(buf)?;
        self.class.pack(buf)?;
        self.ttl.pack(buf)?;
        self.rdlen.pack(buf)
    }
}

impl RHeader {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn set_name(&mut self, name: Name) {
        self.name = name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn set_ty(&mut self, ty: Type) {
        self.ty = ty
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn set_class(&mut self, class: Class) {
        self.class = class
    }

    pub fn ttl(&self) -> u32 {
        self.ttl
    }

    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl
    }

    pub fn rdlen(&self) -> u16 {
        self.rdlen
    }

    pub fn set_rdlen(&mut self, rdlen: u16) {
        self.rdlen = rdlen
    }
}
