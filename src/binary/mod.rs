use crate::binary::error::BinaryError;

pub mod error;

pub enum Endianness {
    Big,
    Little,
}

pub type BinaryResult<T> = Result<T, BinaryError>;

pub fn read_u16(data: &[u8], endianness: Endianness) -> BinaryResult<u16> {
    if data.len() < 2 {
        return Err(BinaryError::new("Slice of bytes too short"));
    }

    match endianness {
        Endianness::Big => {
            let v = ((data[0] as u16) << 8) + (data[1] as u16);
            Ok(v)
        }
        Endianness::Little => {
            let v = ((data[1] as u16) << 8) + (data[0] as u16);
            Ok(v)
        }
    }
}

pub fn read_u32(data: &[u8], endianness: Endianness) -> BinaryResult<u32> {
    if data.len() < 4 {
        return Err(BinaryError::new("Slice of bytes too short"));
    }

    match endianness {
        Endianness::Big => {
            let v = ((data[0] as u32) << 24)
                + ((data[1] as u32) << 16)
                + ((data[2] as u32) << 8)
                + (data[3] as u32);
            Ok(v)
        }
        Endianness::Little => {
            let v = ((data[3] as u32) << 24)
                + ((data[2] as u32) << 16)
                + ((data[1] as u32) << 8)
                + (data[0] as u32);
            Ok(v)
        }
    }
}

pub fn read_u64(data: &[u8], endianness: Endianness) -> BinaryResult<u64> {
    if data.len() < 8 {
        return Err(BinaryError::new("Slice of bytes too short"));
    }

    match endianness {
        Endianness::Big => {
            let v = ((data[0] as u64) << 56)
                + ((data[1] as u64) << 48)
                + ((data[2] as u64) << 40)
                + ((data[3] as u64) << 32)
                + ((data[4] as u64) << 24)
                + ((data[5] as u64) << 16)
                + ((data[6] as u64) << 8)
                + (data[7] as u64);
            Ok(v)
        }
        Endianness::Little => {
            let v = ((data[7] as u64) << 56)
                + ((data[6] as u64) << 48)
                + ((data[5] as u64) << 40)
                + ((data[4] as u64) << 32)
                + ((data[3] as u64) << 24)
                + ((data[2] as u64) << 16)
                + ((data[1] as u64) << 8)
                + (data[0] as u64);
            Ok(v)
        }
    }
}
