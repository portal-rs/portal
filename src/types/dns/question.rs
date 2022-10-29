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

impl Default for Question {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ty: Type::NONE,
            class: Class::IN,
        }
    }
}

impl Unpackable for Question {
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

        Ok(Question { name, ty, class })
    }
}
