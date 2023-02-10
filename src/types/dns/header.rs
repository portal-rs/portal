use binbuf::prelude::*;

use crate::types::{opcode::Opcode, rcode::Rcode};

/// [`Header`] describes the header data of a message. This header format enables easy access to all header fields. The
/// [`RawHeader`] in comparison stores raw data directly from the wire.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub id: u16,
    pub is_query: bool,
    pub opcode: Opcode,
    pub authoritative: bool,
    pub truncated: bool,
    pub rec_des: bool,
    pub rec_avail: bool,
    pub zero: bool,
    pub rcode: Rcode,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            id: 0,
            is_query: true,
            opcode: Opcode::Query,
            authoritative: false,
            truncated: false,
            rec_des: true,
            rec_avail: false,
            zero: false,
            rcode: Rcode::NoError,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
}

impl From<RawHeader> for Header {
    fn from(h: RawHeader) -> Self {
        Self {
            id: h.id,
            is_query: h.is_query(),
            authoritative: h.is_authoritative(),
            opcode: h.opcode(),
            truncated: h.is_truncated(),
            rec_des: h.is_rec_des(),
            rec_avail: h.is_rec_avail(),
            zero: h.is_zero(),
            rcode: h.rcode(),
            qdcount: h.qdcount,
            ancount: h.ancount,
            nscount: h.nscount,
            arcount: h.arcount,
        }
    }
}

impl Readable for Header {
    type Error = BufferError;

    /// Unpacks the first 12 octets from the DNS message. The DNS header is
    /// fixed in size. The function returns the [`Header`] it self and the
    /// offset (which will always be 12). This function is usually the first
    /// step in unpacking the whole message.
    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let id = u16::read::<E>(buf)?;
        let flags = u16::read::<E>(buf)?;
        let qdcount = u16::read::<E>(buf)?;
        let ancount = u16::read::<E>(buf)?;
        let nscount = u16::read::<E>(buf)?;
        let arcount = u16::read::<E>(buf)?;

        let header = Header::from(RawHeader {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        });

        Ok(header)
    }
}

impl Writeable for Header {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let raw_header = RawHeader::from(self);
        raw_header.write::<E>(buf)
    }
}

impl Header {
    /// Construct a new (default) DNS [`Header`] with the provided ID.
    pub fn new(id: u16) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

/// [`RawHeader`] describes the raw header data of a message directly from the wire. The data gets unpacked by splitting
/// the message into six 16 bit (2 octet) chunks. The first chunk is just the **ID**. The second chunk **flags** carries
/// data like QR, OPCODE, etc. which gets split up further by bit masks. The remaining four chunks contain counts for
/// questions, answers, ns and additional records.
pub struct RawHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Writeable for RawHeader {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        self.id.write::<E>(buf)?;
        self.flags.write::<E>(buf)?;
        self.qdcount.write::<E>(buf)?;
        self.ancount.write::<E>(buf)?;
        self.nscount.write::<E>(buf)?;
        self.arcount.write::<E>(buf)
    }
}

impl From<&Header> for RawHeader {
    fn from(header: &Header) -> Self {
        // Start to build flags by appending the OPCODE and RCODE
        let opcode: u16 = header.opcode.into();
        let rcode: u16 = header.rcode.into();
        let mut flags = opcode << 11 | rcode & 0xF;

        if !header.is_query {
            flags |= 1 << 15;
        }

        if header.authoritative {
            flags |= 1 << 10;
        }

        if header.truncated {
            flags |= 1 << 9;
        }

        if header.rec_des {
            flags |= 1 << 8;
        }

        if header.rec_avail {
            flags |= 1 << 7;
        }

        if header.zero {
            flags |= 1 << 6;
        }

        Self {
            qdcount: header.qdcount,
            ancount: header.ancount,
            nscount: header.nscount,
            arcount: header.arcount,
            id: header.id,
            flags,
        }
    }
}

impl RawHeader {
    /// Returns if this DNS message is a query (QR) by applying a bit mask.
    pub fn is_query(&self) -> bool {
        self.flags & (1 << 15) == 0
    }

    /// Returns the OPCODE of the DNS message by applying a bit mask.
    pub fn opcode(&self) -> Opcode {
        Opcode::from((self.flags >> 11) & 0xF)
    }

    /// Returns if the DNS message is authoritative (AA) by applying a bit mask.
    pub fn is_authoritative(&self) -> bool {
        self.flags & (1 << 10) != 0
    }

    /// Returns if the DNS message is truncated (TC) by applying a bit mask.
    pub fn is_truncated(&self) -> bool {
        self.flags & (1 << 9) != 0
    }

    /// Returns if the RD flag is set by applying a bit mask.
    pub fn is_rec_des(&self) -> bool {
        self.flags & (1 << 8) != 0
    }

    /// Returns if the RA flag is set by applying a bit mask.
    pub fn is_rec_avail(&self) -> bool {
        self.flags & (1 << 7) != 0
    }

    /// Returns if the ZERO (Z) bits are set by applying a bit mask.
    pub fn is_zero(&self) -> bool {
        self.flags & (1 << 6) != 0
    }

    /// Returns the OPCODE of the DNS message by applying a bit mask.
    pub fn rcode(&self) -> Rcode {
        Rcode::from(self.flags & 0xF)
    }
}
