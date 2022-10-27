use std::net::{Ipv4Addr, Ipv6Addr};

use crate::packing::UnpackError;

pub struct UnpackBuffer<'a> {
    buf: &'a [u8],
    rest: &'a [u8],
    ptrs: Vec<usize>,
}

pub type UnpackBufferResult<T> = Result<T, UnpackError>;

impl<'a> UnpackBuffer<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        UnpackBuffer {
            buf,
            rest: buf,
            ptrs: Vec::new(),
        }
    }

    pub fn pop(&mut self) -> UnpackBufferResult<u8> {
        if let Some((first, rest)) = self.rest.split_first() {
            self.rest = rest;
            return Ok(*first);
        }

        Err(UnpackError::TooShort)
    }

    pub fn peek(&self) -> Option<u8> {
        match self.rest.first() {
            Some(b) => Some(*b),
            None => None,
        }
    }

    pub fn followed_pointers(&self) -> bool {
        self.ptrs.len() > 0
    }

    pub fn jump_to(&mut self, index: usize) {
        self.ptrs.push(self.offset())
        // Jump
    }

    pub fn jump_back(&mut self) {}

    pub fn offset(&self) -> usize {
        return self.buf.len() - self.rest.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.rest.len() == 0;
    }

    pub fn read_u16(&mut self) -> UnpackBufferResult<u16> {
        if let Ok(b) = self.read_slice(2) {
            let n = u16::from_be_bytes([b[0], b[1]]);
            return Ok(n);
        }

        return Err(UnpackError::TooShort);
    }

    pub fn read_u32(&mut self) -> UnpackBufferResult<u32> {
        if let Ok(b) = self.read_slice(4) {
            let n = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
            return Ok(n);
        }

        return Err(UnpackError::TooShort);
    }

    pub fn read_u64(&mut self) -> UnpackBufferResult<u64> {
        if let Ok(b) = self.read_slice(8) {
            let n = u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
            return Ok(n);
        }

        return Err(UnpackError::TooShort);
    }

    pub fn read_u128(&mut self) -> UnpackBufferResult<u128> {
        if let Ok(b) = self.read_slice(16) {
            let n = u128::from_be_bytes([
                b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12],
                b[13], b[14], b[15],
            ]);
            return Ok(n);
        }

        return Err(UnpackError::TooShort);
    }

    pub fn read_character_string(&mut self, max_len: u8) -> UnpackBufferResult<&[u8]> {
        let len = match self.pop() {
            Ok(len) if len <= max_len => len,
            _ => return Err(UnpackError::TooShort),
        };

        return self.read_slice(len as usize);
    }

    pub fn read_ipv4_address(&mut self) -> UnpackBufferResult<Ipv4Addr> {
        if let Ok(n) = self.read_u32() {
            let ip_addr = Ipv4Addr::from(n);
            return Ok(ip_addr);
        }

        return Err(UnpackError::TooShort);
    }

    pub fn read_ipv6_address(&mut self) -> UnpackBufferResult<Ipv6Addr> {
        if let Ok(n) = self.read_u128() {
            let ip_addr = Ipv6Addr::from(n);
            return Ok(ip_addr);
        }

        return Err(UnpackError::TooShort);
    }

    pub fn read_slice(&mut self, nbytes: usize) -> UnpackBufferResult<&'a [u8]> {
        if nbytes > self.rest.len() {
            return Err(UnpackError::TooShort);
        }

        let (slice, rest) = self.rest.split_at(nbytes);
        self.rest = rest;
        return Ok(slice);
    }
}

pub trait Unpackable: Sized {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self>;
    // fn unpack_iter(buf: &mut UnpackBuffer, n: usize) -> UnpackBufferResult<Vec<Self>>;
}
