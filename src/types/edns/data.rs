use crate::packing::{UnpackBuffer, UnpackBufferResult};

#[derive(Debug)]
pub enum OptionData {}

impl OptionData {
    pub fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        todo!()
    }
}
