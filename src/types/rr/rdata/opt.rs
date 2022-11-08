use std::collections::HashMap;

use crate::{
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
    types::{
        edns::{EdnsHeader, Option, OptionCode},
        rr::RHeader,
    },
};

#[derive(Debug)]
pub struct OPT {
    header: EdnsHeader,
    options: HashMap<OptionCode, Option>,
}

impl OPT {
    pub fn unpack(buf: &mut UnpackBuffer, rheader: &RHeader) -> UnpackBufferResult<Self> {
        // First we create the EDNS header
        let header = EdnsHeader::from(rheader);

        // Setup unpacking of EDNS options
        let start_len = buf.len();
        let rdlen = rheader.rdlen as usize;
        let mut options = HashMap::new();

        // Unpack uptions until rdlen is exhausted
        while start_len - buf.len() < rdlen {
            let option = Option::unpack(buf)?;
            options.insert(option.code(), option);
        }

        Ok(Self { header, options })
    }
}

impl Packable for OPT {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        for (_, option) in &self.options {
            option.pack(buf)?;
        }

        Ok(())
    }
}
