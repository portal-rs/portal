use crate::binary::error::BinaryError;

pub mod error;

pub enum Endianness {
    Big,
    Little,
}

pub type BinaryReadResult<T> = Result<T, BinaryError>;
pub type BinaryWriteResult = Result<(), BinaryError>;

pub fn read_u16(data: &[u8], endianness: Endianness) -> BinaryReadResult<u16> {
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

pub fn read_u32(data: &[u8], endianness: Endianness) -> BinaryReadResult<u32> {
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

pub fn read_u64(data: &[u8], endianness: Endianness) -> BinaryReadResult<u64> {
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

pub fn put_u16(value: u16, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < 2 {
        return Err(BinaryError::new("Buf too short"));
    }

    match endianness {
        Endianness::Big => {
            buf[0] = (value >> 8) as u8;
            buf[1] = value as u8;
        }
        Endianness::Little => {
            buf[0] = value as u8;
            buf[1] = (value >> 8) as u8;
        }
    }

    Ok(())
}

pub fn put_u32(value: u32, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < 4 {
        return Err(BinaryError::new("Buf too short"));
    }

    match endianness {
        Endianness::Big => {
            buf[0] = (value >> 24) as u8;
            buf[1] = (value >> 16) as u8;
            buf[2] = (value >> 8) as u8;
            buf[3] = value as u8;
        }
        Endianness::Little => {
            buf[0] = value as u8;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
        }
    }

    Ok(())
}

pub fn put_u64(value: u64, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < 8 {
        return Err(BinaryError::new("Buf too short"));
    }

    match endianness {
        Endianness::Big => {
            buf[0] = (value >> 56) as u8;
            buf[1] = (value >> 48) as u8;
            buf[2] = (value >> 40) as u8;
            buf[3] = (value >> 32) as u8;
            buf[4] = (value >> 24) as u8;
            buf[5] = (value >> 16) as u8;
            buf[6] = (value >> 8) as u8;
            buf[7] = value as u8;
        }
        Endianness::Little => {
            buf[0] = value as u8;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
            buf[4] = (value >> 32) as u8;
            buf[5] = (value >> 40) as u8;
            buf[6] = (value >> 48) as u8;
            buf[7] = (value >> 56) as u8;
        }
    }

    Ok(())
}
