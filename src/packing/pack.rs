use crate::{binary, packing::error::PackError};

pub type PackResult = Result<usize, PackError>;

/// Savely packs a u16 into a vector of bytes at `offset`.
pub fn pack_u16(value: u16, buf: &mut Vec<u8>, offset: usize) -> PackResult {
    if offset + 2 > buf.len() {
        return Err(PackError::new("Offset overflow unpacking u16"));
    }

    return match binary::put_u16(value, &mut buf[offset..], binary::Endianness::Big) {
        Ok(_) => Ok(offset + 2),
        Err(_) => Err(PackError::new("Slice too short to unpack u16")),
    };
}

/// Savely packs a u32 into a vector of bytes.
pub fn pack_u32(value: u32, buf: &mut Vec<u8>, offset: usize) -> PackResult {
    if offset + 4 > buf.len() {
        return Err(PackError::new("Offset overflow unpacking u32"));
    }

    return match binary::put_u32(value, &mut buf[offset..], binary::Endianness::Big) {
        Ok(_) => Ok(offset + 4),
        Err(_) => Err(PackError::new("Slice too short to unpack u32")),
    };
}

/// Savely packs a u64 into a vector of bytes.
pub fn pack_u64(value: u64, buf: &mut Vec<u8>, offset: usize) -> PackResult {
    if offset + 8 > buf.len() {
        return Err(PackError::new("Offset overflow unpacking u64"));
    }

    return match binary::put_u64(value, &mut buf[offset..], binary::Endianness::Big) {
        Ok(_) => Ok(offset + 8),
        Err(_) => Err(PackError::new("Slice too short to unpack u64")),
    };
}
