use crate::{
    packing::{UnpackBuffer, UnpackBufferResult, Unpackable},
    types::dns::Name,
};

#[derive(Debug)]
pub struct MINFO {
    rmailbx: Name,
    emailbx: Name,
}

impl Unpackable for MINFO {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let rmailbx = Name::unpack(buf)?;
        let emailbx = Name::unpack(buf)?;

        Ok(Self { rmailbx, emailbx })
    }
}
