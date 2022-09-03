use crate::{binary, pack::error::UnpackError, types::dns::header::Header};

pub type UnpackResult<T> = Result<(T, usize), UnpackError>;

/// Savely unpacks a u16 from a vector of bytes.
pub fn unpack_u16(data: Vec<u8>, offset: usize) -> UnpackResult<u16> {
    return match binary::read_u16(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 2)),
        Err(_) => Err(UnpackError::new("Offset overflow unpacking u16")),
    };
}

/// Savely unpacks a u32 from a vector of bytes.
pub fn unpack_u32(data: Vec<u8>, offset: usize) -> UnpackResult<u32> {
    return match binary::read_u32(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 4)),
        Err(_) => Err(UnpackError::new("Offset overflow unpacking u32")),
    };
}

/// Savely unpacks a u64 from a vector of bytes.
pub fn unpack_u64(data: Vec<u8>, offset: usize) -> UnpackResult<u64> {
    return match binary::read_u64(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 8)),
        Err(_) => Err(UnpackError::new("Offset overflow unpacking u64")),
    };
}

pub fn unpack_header(data: Vec<u8>) -> Result<(Header, usize), UnpackError> {}
