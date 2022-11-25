/// The compression pointer mask masks the first to bits of the label length / ptr bytes.
pub const COMPRESSION_POINTER_MASK: u16 = 0x3FFF;

/// The maximum character string length is 255 octets.
pub const MAX_CHAR_STRING_LENGTH: u8 = 255;

/// DNS questions always have a minimum fixed length of 2 octects for QTYPE and QCLASS.
pub const QUESTION_FIXED_LENGTH: usize = 4;

/// The maximum domain length is 255 octets.
pub const MAX_DOMAIN_LENGTH: u8 = 255;

/// The maximum length for a single domain label is 63 octets.
pub const MAX_LABEL_LENGTH: u8 = 0x3F;

/// The DNS message header is always 12 octets long.
pub const HEADER_LENGTH: usize = 12;
