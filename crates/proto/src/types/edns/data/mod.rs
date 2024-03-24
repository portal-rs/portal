use crate::types::edns::OptionCode;

mod cookie;

use binbuf::{
    read::{ReadBuffer, ReadError},
    write::{WriteBuffer, WriteError, Writeable},
    Endianness,
};
use cookie::*;

#[derive(Debug, Clone)]
pub enum OptionData {
    COOKIE(COOKIE),
}

impl OptionData {
    pub fn read<E: Endianness>(
        buf: &mut ReadBuffer,
        opt_code: OptionCode,
        len: u16,
    ) -> Result<Self, ReadError> {
        match opt_code {
            OptionCode::RESERVED(_) => todo!(),
            OptionCode::RESERVEDLOCAL(_) => todo!(),
            OptionCode::UNASSIGNED => todo!(),
            OptionCode::LLQ => todo!(),
            OptionCode::UL => todo!(),
            OptionCode::NSID => todo!(),
            OptionCode::DAU => todo!(),
            OptionCode::DHU => todo!(),
            OptionCode::N3U => todo!(),
            OptionCode::ECS => todo!(),
            OptionCode::EXPIRE => todo!(),
            OptionCode::COOKIE => COOKIE::read::<E>(buf, len).map(Self::COOKIE),
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

impl Writeable for OptionData {
    type Error = WriteError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        match self {
            OptionData::COOKIE(c) => c.write::<E>(buf),
        }
    }
}
