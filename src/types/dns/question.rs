use crate::{
    packing::{UnpackBuffer, UnpackBufferResult, Unpackable},
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
