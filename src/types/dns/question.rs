use std::fmt::Display;

use binbuf::prelude::*;

use crate::{
    constants,
    types::{
        dns::{Name, Query},
        rr::{Class, Type},
    },
};

/// [`Question`] describes a DNS question. The RFC allows multiple questions
/// per message, but most DNS servers only accept one and multiple questions
/// often result in errors.
///
/// ### Further information
///
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2
#[derive(Debug, Clone)]
pub struct Question {
    pub name: Name,
    pub ty: Type,
    pub class: Class,
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}\t{}", self.name, self.class, self.ty)
    }
}

impl From<Query> for Question {
    fn from(q: Query) -> Self {
        Question {
            name: q.name,
            ty: q.ty,
            class: q.class,
        }
    }
}

impl Readable for Question {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let name = Name::read::<E>(buf)?;
        let ty = Type::read::<E>(buf)?;
        let class = Class::read::<E>(buf)?;

        Ok(Question { name, ty, class })
    }
}

impl Writeable for Question {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.name.write::<E>(buf)?;
            self.ty.write::<E>(buf)?;
            self.class.write::<E>(buf)?
        };

        Ok(n)
    }
}

impl Question {
    /// Returns the length of this [`Question`] by adding up the length of the
    /// domain name and the fixed length (QTYPE and QCLASS).
    pub fn len(&self) -> usize {
        self.name.len() + constants::dns::QUESTION_FIXED_LENGTH
    }
}
