use crate::{
    packing::{UnpackBuffer, UnpackBufferResult, Unpackable},
    types::dns::Name,
};

pub fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Name> {
    Name::unpack(buf)
}
