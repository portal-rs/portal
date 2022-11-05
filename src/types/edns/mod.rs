use crate::packing::{UnpackBuffer, UnpackBufferResult, Unpackable};

mod code;
mod data;
mod header;

pub use code::*;
pub use data::*;
pub use header::*;

#[derive(Debug)]
pub struct Option {
    // This is redundant data. We already store the code in the map
    code: OptionCode,
    data: OptionData,
    len: u16,
}

impl Unpackable for Option {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let code = OptionCode::unpack(buf)?;
        let len = u16::unpack(buf)?;
        let data = OptionData::unpack(buf)?;

        Ok(Option { code, data, len })
    }
}
