use std::net::{Ipv4Addr, Ipv6Addr};

use crate::packing::PackingError;

pub struct PackBuffer {
    buf: Vec<u8>,
}

pub type PackBufferResult = Result<(), PackingError>;

impl PackBuffer {
    pub fn new() -> Self {
        PackBuffer { buf: Vec::new() }
    }

    pub fn len(&self) -> usize {
        return self.buf.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.buf.len() == 0;
    }

    pub fn push(&mut self, b: u8) {
        self.buf.push(b);
    }

    pub fn pack_slice(&mut self, s: &[u8]) -> PackBufferResult {
        self.buf.extend_from_slice(s);
        Ok(())
    }

    pub fn pack_vec(&mut self, v: &mut Vec<u8>) -> PackBufferResult {
        self.buf.append(v);
        Ok(())
    }

    pub fn bytes(&self) -> &[u8] {
        return self.buf.as_slice();
    }
}

pub trait Packable: Sized {
    /// Pack type into a [`PackBuffer`].
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult;
}

impl Packable for u16 {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let b = self.to_be_bytes();
        buf.pack_slice(&b[..])
    }
}

impl Packable for u32 {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let b = self.to_be_bytes();
        buf.pack_slice(&b[..])
    }
}

impl Packable for u64 {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let b = self.to_be_bytes();
        buf.pack_slice(&b[..])
    }
}

impl Packable for u128 {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let b = self.to_be_bytes();
        buf.pack_slice(&b[..])
    }
}

impl Packable for Ipv4Addr {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let b = self.octets();
        buf.pack_slice(&b[..])
    }
}

impl Packable for Ipv6Addr {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let b = self.octets();
        buf.pack_slice(&b[..])
    }
}
