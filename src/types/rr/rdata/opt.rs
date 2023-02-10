use std::{collections::HashMap, fmt::Display};

use binbuf::prelude::*;

use crate::types::{
    edns::{EdnsHeader, Option, OptionCode},
    rr::RHeader,
};

#[derive(Debug, Clone)]
pub struct OPT {
    header: EdnsHeader,
    options: HashMap<OptionCode, Option>,
}

impl Display for OPT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl OPT {
    pub fn read<E: Endianness>(
        buf: &mut ReadBuffer,
        rheader: &RHeader,
    ) -> Result<Self, BufferError> {
        // First we create the EDNS header
        let header = EdnsHeader::from(rheader);

        // Setup unpacking of EDNS options
        let start_len = buf.len();
        let rdlen = rheader.rdlen() as usize;
        let mut options = HashMap::new();

        // Unpack options until rdlen is exhausted
        while start_len - buf.len() < rdlen {
            let option = Option::read::<E>(buf)?;
            options.insert(option.code(), option);
        }

        Ok(Self { header, options })
    }
}

impl Writeable for OPT {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let mut n = 0;

        for option in self.options.values() {
            n += option.write::<E>(buf)?;
        }

        Ok(n)
    }
}
