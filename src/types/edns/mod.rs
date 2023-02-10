use binbuf::prelude::*;

mod code;
mod data;
mod header;

pub use code::*;
pub use data::*;
pub use header::*;

#[derive(Debug, Clone)]
pub struct Option {
    // This is redundant data. We already store the code in the map
    code: OptionCode,
    data: OptionData,
    len: u16,
}

impl Readable for Option {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let code = OptionCode::read::<E>(buf)?;
        let len = u16::read::<E>(buf)?;
        let data = OptionData::read::<E>(buf, code, len)?;

        Ok(Option { code, data, len })
    }
}

impl Writeable for Option {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.code.write::<E>(buf)?;
            self.len.write::<E>(buf)?;
            self.data.write::<E>(buf)?
        };

        Ok(n)
    }
}

impl Option {
    pub fn code(&self) -> OptionCode {
        self.code
    }

    pub fn len(&self) -> u16 {
        self.len
    }
}
