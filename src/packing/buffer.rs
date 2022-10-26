use std::net::{Ipv4Addr, Ipv6Addr};

pub struct UnpackBuffer<'a> {
    buf: &'a [u8],
    rest: &'a [u8],
}

pub struct BufferError {}

pub type BufferResult<T> = Result<T, BufferError>;

impl<'a> UnpackBuffer<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        UnpackBuffer { buf, rest: buf }
    }

    pub fn pop(&mut self) -> BufferResult<u8> {
        if let Some((first, rest)) = self.rest.split_first() {
            self.rest = rest;
            return Ok(*first);
        }

        return Err(BufferError {});
    }

    pub fn offset(&self) -> usize {
        return self.buf.len() - self.rest.len();
    }

    pub fn read_u16(&mut self) -> BufferResult<u16> {
        if let Ok(b) = self.read_slice(2) {
            let n = u16::from_be_bytes([b[0], b[1]]);
            return Ok(n);
        }

        return Err(BufferError {});
    }

    pub fn read_u32(&mut self) -> BufferResult<u32> {
        if let Ok(b) = self.read_slice(4) {
            let n = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
            return Ok(n);
        }

        return Err(BufferError {});
    }

    pub fn read_u64(&mut self) -> BufferResult<u64> {
        if let Ok(b) = self.read_slice(8) {
            let n = u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
            return Ok(n);
        }

        return Err(BufferError {});
    }

    pub fn read_u128(&mut self) -> BufferResult<u128> {
        if let Ok(b) = self.read_slice(16) {
            let n = u128::from_be_bytes([
                b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12],
                b[13], b[14], b[15],
            ]);
            return Ok(n);
        }

        return Err(BufferError {});
    }

    pub fn read_character_string(&mut self) -> BufferResult<&[u8]> {
        let len = match self.pop() {
            Ok(len) if len <= 256 => len,
            _ => return Err(BufferError {}),
        };

        return self.read_slice(len as usize);
    }

    pub fn read_ipv4_address(&mut self) -> BufferResult<Ipv4Addr> {
        if let Ok(n) = self.read_u32() {
            let ip_addr = Ipv4Addr::from(n);
            return Ok(ip_addr);
        }

        return Err(BufferError {});
    }

    pub fn read_ipv6_address(&mut self) -> BufferResult<Ipv6Addr> {
        if let Ok(n) = self.read_u128() {
            let ip_addr = Ipv6Addr::from(n);
            return Ok(ip_addr);
        }

        return Err(BufferError {});
    }

    pub fn read_slice(&mut self, nbytes: usize) -> BufferResult<&'a [u8]> {
        if nbytes > self.rest.len() {
            return Err(BufferError {});
        }

        let (slice, rest) = self.rest.split_at(nbytes);
        self.rest = rest;
        return Ok(slice);
    }
}

pub trait Unpackable: Sized {
    fn unpack(buf: &mut UnpackBuffer) -> BufferResult<Self>;
}
