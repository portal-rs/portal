use std::net::Ipv4Addr;

use crate::packing::{UnpackBuffer, UnpackBufferResult, Unpackable};

pub fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Ipv4Addr> {
    Ipv4Addr::unpack(buf)
}
