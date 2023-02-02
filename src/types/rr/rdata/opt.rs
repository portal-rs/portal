use std::{collections::HashMap, fmt::Display};

use crate::{
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::{
        edns::{EdnsHeader, Option, OptionCode},
        rr::RHeader,
    },
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
    pub fn unpack(buf: &mut UnpackBuffer, rheader: &RHeader) -> UnpackBufferResult<Self> {
        // First we create the EDNS header
        let header = EdnsHeader::from(rheader);

        // Setup unpacking of EDNS options
        let start_len = buf.len();
        let rdlen = rheader.rdlen() as usize;
        let mut options = HashMap::new();

        // Unpack options until rdlen is exhausted
        while start_len - buf.len() < rdlen {
            let option = Option::unpack(buf)?;
            options.insert(option.code(), option);
        }

        Ok(Self { header, options })
    }
}

impl Packable for OPT {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        for option in self.options.values() {
            option.pack(buf)?;
        }

        Ok(())
    }
}
