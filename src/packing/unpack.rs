use crate::{
    binary,
    packing::error::UnpackError,
    types::dns::header::{Header, RawHeader},
};

pub type UnpackResult<T> = Result<(T, usize), UnpackError>;

/// Savely unpacks a u16 from a vector of bytes.
pub fn unpack_u16(data: &Vec<u8>, offset: usize) -> UnpackResult<u16> {
    if offset + 2 > data.len() {
        return Err(UnpackError::new("Offset overflow unpacking u16"));
    }

    return match binary::read_u16(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 2)),
        Err(_) => Err(UnpackError::new("Slice too short to unpack u16")),
    };
}

/// Savely unpacks a u32 from a vector of bytes.
pub fn unpack_u32(data: &Vec<u8>, offset: usize) -> UnpackResult<u32> {
    if offset + 4 > data.len() {
        return Err(UnpackError::new("Offset overflow unpacking u32"));
    }

    return match binary::read_u32(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 4)),
        Err(_) => Err(UnpackError::new("Slice too short to unpack u32")),
    };
}

/// Savely unpacks a u64 from a vector of bytes.
pub fn unpack_u64(data: &Vec<u8>, offset: usize) -> UnpackResult<u64> {
    if offset + 8 > data.len() {
        return Err(UnpackError::new("Offset overflow unpacking u64"));
    }

    return match binary::read_u64(&data[offset..], binary::Endianness::Big) {
        Ok(int) => Ok((int, offset + 8)),
        Err(_) => Err(UnpackError::new("Slice too short to unpack u64")),
    };
}

/// Unpacks the DNS header
pub fn unpack_header(data: Vec<u8>) -> Result<(Header, usize), UnpackError> {
    let (id, offset) = match unpack_u16(&data, 0) {
        Ok(id) => id,
        Err(_) => todo!(),
    };

    let (flags, offset) = match unpack_u16(&data, offset) {
        Ok(flags) => flags,
        Err(_) => todo!(),
    };

    let (qdcount, offset) = match unpack_u16(&data, offset) {
        Ok(qdcount) => qdcount,
        Err(_) => todo!(),
    };

    let (ancount, offset) = match unpack_u16(&data, offset) {
        Ok(ancount) => ancount,
        Err(_) => todo!(),
    };

    let (nscount, offset) = match unpack_u16(&data, offset) {
        Ok(nscount) => nscount,
        Err(_) => todo!(),
    };

    let (arcount, offset) = match unpack_u16(&data, offset) {
        Ok(arcount) => arcount,
        Err(_) => todo!(),
    };

    let header = Header::from(RawHeader {
        id,
        flags,
        qdcount,
        ancount,
        nscount,
        arcount,
    });

    Ok((header, offset))
}
