use std::net::{Ipv4Addr, Ipv6Addr};

use crate::error::ProtocolError;

pub struct UnpackBuffer<'a> {
    buf: &'a [u8],
    rest: &'a [u8],
    ptrs: Vec<usize>,
}

pub type UnpackBufferResult<T> = Result<T, ProtocolError>;

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

        Err(ProtocolError::BufTooShort)
    }

    pub fn peek(&self) -> Option<u8> {
        self.rest.first().map(|b| *b)
    }

    pub fn followed_pointers(&self) -> bool {
        !self.ptrs.is_empty()
    }

    pub fn jump_to(&mut self, index: usize) -> UnpackBufferResult<()> {
        if index > self.buf.len() {
            return Err(ProtocolError::BufTooShort);
        }

        self.ptrs.push(self.offset());
        self.rest = &self.buf[index..];

        Ok(())
    }

    pub fn jump_back(&mut self) {
        if let Some(index) = self.ptrs.pop() {
            self.rest = &self.buf[index..];
        }
    }

    pub fn iter_back(&mut self) -> bool {
        if !self.followed_pointers() {
            return false;
        }

        while self.followed_pointers() {
            self.jump_back()
        }

        true
    }

    pub fn offset(&self) -> usize {
        self.buf.len() - self.rest.len()
    }

    pub fn len(&self) -> usize {
        self.rest.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rest.len() == 0
    }

    pub fn unpack_character_string(&mut self, max_len: u8) -> UnpackBufferResult<&'a [u8]> {
        let len = match self.pop() {
            Ok(len) => {
                if len > max_len {
                    return Err(ProtocolError::CharStringExceededMaxLen(max_len));
                }
                len
            }
            _ => return Err(ProtocolError::BufTooShort),
        };

        return self.unpack_slice(len as usize);
    }

    pub fn unpack_slice(&mut self, nbytes: usize) -> UnpackBufferResult<&'a [u8]> {
        if nbytes > self.len() {
            return Err(ProtocolError::BufTooShort);
        }

        let (slice, rest) = self.rest.split_at(nbytes);
        self.rest = rest;

        Ok(slice)
    }

    pub fn unpack_vec(&mut self, nbytes: usize) -> UnpackBufferResult<Vec<u8>> {
        self.unpack_slice(nbytes).map(ToOwned::to_owned)
    }
}

pub trait Unpackable: Sized {
    /// Unpack type from an [`UnpackBuffer`].
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self>;
}

impl Unpackable for u16 {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        if let Ok(b) = buf.unpack_slice(2) {
            let n = u16::from_be_bytes([b[0], b[1]]);
            return Ok(n);
        }

        Err(ProtocolError::BufTooShort)
    }
}

impl Unpackable for u32 {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        if let Ok(b) = buf.unpack_slice(4) {
            let n = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
            return Ok(n);
        }

        Err(ProtocolError::BufTooShort)
    }
}

impl Unpackable for u64 {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        if let Ok(b) = buf.unpack_slice(8) {
            let n = u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
            return Ok(n);
        }

        Err(ProtocolError::BufTooShort)
    }
}

impl Unpackable for u128 {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        if let Ok(b) = buf.unpack_slice(16) {
            let n = u128::from_be_bytes([
                b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12],
                b[13], b[14], b[15],
            ]);
            return Ok(n);
        }

        Err(ProtocolError::BufTooShort)
    }
}

impl Unpackable for Ipv4Addr {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        if let Ok(n) = u32::unpack(buf) {
            let ip_addr = Ipv4Addr::from(n);
            return Ok(ip_addr);
        }

        Err(ProtocolError::BufTooShort)
    }
}

impl Unpackable for Ipv6Addr {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        if let Ok(n) = u128::unpack(buf) {
            let ip_addr = Ipv6Addr::from(n);
            return Ok(ip_addr);
        }

        Err(ProtocolError::BufTooShort)
    }
}
