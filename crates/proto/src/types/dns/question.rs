use std::fmt::Display;

use binbuf::{
    macros::bytes_written,
    read::{ReadBuffer, ReadError, Readable},
    write::{WriteBuffer, WriteError, Writeable},
    Endianness,
};
use snafu::{ResultExt, Snafu};

use crate::{
    constants,
    types::{
        dns::{Name, NameError, Query},
        rr::{Class, RType},
    },
};

#[derive(Debug, Snafu)]
pub enum QuestionError {
    #[snafu(display("failed to read QNAME"))]
    ReadName { source: NameError },

    #[snafu(display("failed to write QNAME"))]
    WriteName { source: NameError },

    #[snafu(display("failed to read QTYPE"))]
    ReadType { source: ReadError },

    #[snafu(display("failed to write QTYPE"))]
    WriteType { source: WriteError },

    #[snafu(display("failed to read QCLASS"))]
    ReadClass { source: ReadError },

    #[snafu(display("failed to write QCLASS"))]
    WriteClass { source: WriteError },
}

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
    pub ty: RType,
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
    type Error = QuestionError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let name = Name::read::<E>(buf).context(ReadNameSnafu)?;
        let ty = RType::read::<E>(buf).context(ReadTypeSnafu)?;
        let class = Class::read::<E>(buf).context(ReadClassSnafu)?;

        Ok(Question { name, ty, class })
    }
}

impl Writeable for Question {
    type Error = QuestionError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.name.write::<E>(buf).context(WriteNameSnafu)?;
            self.ty.write::<E>(buf).context(WriteTypeSnafu)?;
            self.class.write::<E>(buf).context(WriteClassSnafu)?
        };

        Ok(n)
    }
}

impl Question {
    pub fn new(name: Name, ty: RType, class: Class) -> Self {
        Self { name, ty, class }
    }

    /// Returns the size of this [`Question`] by adding up the length of the
    /// domain name and the fixed length (QTYPE and QCLASS).
    pub fn size(&self) -> usize {
        self.name.size() + constants::QUESTION_FIXED_LENGTH
    }
}
