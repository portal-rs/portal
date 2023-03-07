use binbuf::prelude::*;
use serde::Serialize;
use thiserror::Error;

use crate::{
    constants::dns::RECORD_FIXED_LENGTH,
    types::{
        dns::{Name, NameError},
        rr::{Class, Type},
    },
};

#[derive(Debug, Error)]
pub enum RHeaderError {
    #[error("Name error: {0}")]
    NameError(#[from] NameError),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RHeader {
    name: Name,
    ty: Type,
    class: Class,
    ttl: u32,
    rdlen: u16,
}

impl ToString for RHeader {
    fn to_string(&self) -> String {
        format!(
            "N: {} T: {} C: {} TTL: {} ({})",
            self.name, self.ty, self.class, self.ttl, self.rdlen
        )
    }
}

impl Readable for RHeader {
    type Error = RHeaderError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let name = Name::read::<E>(buf)?;
        let ty = Type::read::<E>(buf)?;
        let class = Class::read::<E>(buf)?;
        let ttl = u32::read::<E>(buf)?;
        let rdlen = u16::read::<E>(buf)?;

        Ok(Self {
            name,
            ty,
            class,
            ttl,
            rdlen,
        })
    }
}

impl Writeable for RHeader {
    type Error = RHeaderError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.name.write::<E>(buf)?;
            self.ty.write::<E>(buf)?;
            self.class.write::<E>(buf)?;
            self.ttl.write::<E>(buf)?;
            self.rdlen.write::<E>(buf)?
        };

        Ok(n)
    }
}

impl RHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        RECORD_FIXED_LENGTH + self.name.size()
    }

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
