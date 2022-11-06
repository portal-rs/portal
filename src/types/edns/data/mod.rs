use crate::{
    packing::{UnpackBuffer, UnpackBufferResult},
    types::edns::OptionCode,
};

mod cookie;

use cookie::*;

#[derive(Debug)]
pub enum OptionData {
    COOKIE(COOKIE),
}

impl OptionData {
    pub fn unpack(
        buf: &mut UnpackBuffer,
        opt_code: OptionCode,
        len: u16,
    ) -> UnpackBufferResult<Self> {
        match opt_code {
            OptionCode::RESERVED => todo!(),
            OptionCode::RESERVEDLOCAL => todo!(),
            OptionCode::UNASSIGNED => todo!(),
            OptionCode::LLQ => todo!(),
            OptionCode::UL => todo!(),
            OptionCode::NSID => todo!(),
            OptionCode::DAU => todo!(),
            OptionCode::DHU => todo!(),
            OptionCode::N3U => todo!(),
            OptionCode::ECS => todo!(),
            OptionCode::EXPIRE => todo!(),
            OptionCode::COOKIE => COOKIE::unpack(buf, len).map(Self::COOKIE),
            OptionCode::TCPKEEPALIVE => todo!(),
            OptionCode::PADDING => todo!(),
            OptionCode::CHAIN => todo!(),
            OptionCode::KEYTAG => todo!(),
            OptionCode::EDE => todo!(),
            OptionCode::CLIENTTAG => todo!(),
            OptionCode::SERVERTAG => todo!(),
            OptionCode::UMBRELLAIDENT => todo!(),
            OptionCode::DEVICEID => todo!(),
        }
    }
}
