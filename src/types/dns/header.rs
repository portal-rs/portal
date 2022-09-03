use crate::types::{opcode::Opcode, rcode::Rcode};

/// [`Header`] describes the header data of a message. This header format enables easy access to all header fields. The
/// [`RawHeader`] in comparison stores raw data directly from the wire.
///
/// ### Further information
///
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
pub struct Header {
    id: u16,
    is_query: bool,
    opcode: Opcode,
    authoritative: bool,
    truncated: bool,
    rec_des: bool,
    rec_avail: bool,
    zero: bool,
    rcode: Rcode,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl Header {
    /// Construct a new (default) DNS [`Header`] with the provided ID.
    pub fn new(id: u16) -> Self {
        return Self {
            id,
            ..Default::default()
        };
    }
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
        return Self {
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
        };
    }
}

/// [`RawHeader`] describes the raw header data of a message directly from the wire. The data gets unpacked by splitting
/// the message into six 16 bit (2 octet) chunks. The first chunk is just the **ID**. The second chunk **flags** carries
/// data like QR, OPCODE, etc. which gets split up further by bit masks. The remaining four chunks contain counts for
/// questions, answers, nameserver and additional records.
pub struct RawHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl RawHeader {
    /// Returns if this DNS message is a query (QR) by applying a bit mask.
    pub fn is_query(self) -> bool {
        return self.flags & (1 << 15) == 0;
    }

    /// Returns the OPCODE of the DNS message by applying a bit mask.
    pub fn opcode(self) -> Opcode {
        return Opcode::from((self.flags >> 11) & 0xF);
    }

    /// Returns if the DNS message is authoritative (AA) by applying a bit mask.
    pub fn is_authoritative(self) -> bool {
        return self.flags & (1 << 10) != 0;
    }

    /// Returns if the DNS message is truncated (TC) by applying a bit mask.
    pub fn is_truncated(self) -> bool {
        return self.flags & (1 << 9) != 0;
    }

    /// Returns if the RD flag is set by applying a bit mask.
    pub fn is_rec_des(self) -> bool {
        return self.flags & (1 << 8) != 0;
    }

    /// Returns if the RA flag is set by applying a bit mask.
    pub fn is_rec_avail(self) -> bool {
        return self.flags & (1 << 7) != 0;
    }

    /// Returns if the ZERO (Z) bits are set by applying a bit mask.
    pub fn is_zero(self) -> bool {
        return self.flags & (1 << 6) != 0;
    }

    /// Returns the OPCODE of the DNS message by applying a bit mask.
    pub fn rcode(self) -> Rcode {
        return Rcode::from(self.flags & 0xF);
    }
}
