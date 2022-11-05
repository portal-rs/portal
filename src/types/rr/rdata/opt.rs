use std::collections::HashMap;

use crate::{
    packing::{UnpackBuffer, UnpackBufferResult},
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
        let header = EdnsHeader::from(rheader);

        Ok(Self {
            header,
            options: HashMap::new(),
        })
    }
}
