use crate::{
    constants,
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::{
        dns::Name,
        rr::{Class, Type},
    },
};

/// [`Question`] describes a DNS question. The RFC allows multiple questions per message, but most DNS servers only
/// accpet one and multiple questions often result in errors.
///
/// ### Further information
///
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2
#[derive(Debug)]
pub struct Question {
    pub name: Name,
    pub ty: Type,
    pub class: Class,
}

impl Unpackable for Question {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let name = Name::unpack(buf)?;
        let ty = Type::unpack(buf)?;
        let class = Class::unpack(buf)?;

        Ok(Question { name, ty, class })
    }
}

impl Packable for Question {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        self.name.pack(buf)?;
        self.ty.pack(buf)?;
        self.class.pack(buf)?;

        Ok(())
    }
}

impl Question {
    /// Returns the length of this [`Question`] by adding up the length of the
    /// domain name and the fixed length (QTYPE and QCLASS).
    pub fn len(&self) -> usize {
        return self.name.len() + constants::dns::QUESTION_FIXED_LENGTH;
    }
}
