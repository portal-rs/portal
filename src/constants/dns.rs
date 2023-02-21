/// The compression pointer mask masks off the first two bits of the label length / ptr bytes.
pub const COMP_PTR_MASK: u16 = 0x3FFF;

/// The compression pointer mask masks the last 6 bits of a u8 to extract the upper two bits.
pub const COMP_PTR: u8 = 0xC0;

/// The maximum character string length is 255 octets.
pub const MAX_CHAR_STRING_LENGTH: u8 = 255;

/// DNS questions always have a minimum fixed length of 2 octets for QTYPE and QCLASS.
pub const QUESTION_FIXED_LENGTH: usize = 4;

/// The maximum domain length is 255 octets.
pub const MAX_DOMAIN_LENGTH: u8 = 255;

/// The maximum length for a single domain label is 63 octets.
pub const MAX_LABEL_LENGTH: u8 = 0x3F;

/// The DNS message header is always 12 octets long.
pub const HEADER_LENGTH: usize = 12;
