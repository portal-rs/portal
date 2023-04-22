/// The compression pointer mask masks off the first two bits of the label
/// length / ptr bytes.
pub const COMP_PTR_MASK: u16 = 0x3FFF;

/// The compression pointer mask masks the last 6 bits of a u8 to extract the
/// upper two bits.
pub const COMP_PTR: u8 = 0xC0;

/// The maximum character string length is 255 octets.
pub const MAX_CHAR_STRING_LENGTH: u8 = 255;

/// DNS questions always have a minimum fixed length of 2 octets for QTYPE and
/// QCLASS.
pub const QUESTION_FIXED_LENGTH: usize = 4;

/// DNS records always have a minimum fixed length of 10 octets for TYPE (2),
/// CLASS (2), TTL (4) and RDLEN (2).
pub const RECORD_FIXED_LENGTH: usize = 10;

/// The maximum domain length is 255 octets.
pub const MAX_DOMAIN_LENGTH: u8 = 255;

/// The maximum length for a single domain label is 63 octets.
pub const MAX_LABEL_LENGTH: u8 = 0x3F;

/// The DNS message header is always 12 octets long.
pub const HEADER_LENGTH: usize = 12;

pub const MIN_MESSAGE_SIZE: usize = 512;
pub const MAX_MESSAGE_SIZE: usize = u16::MAX as usize;

pub const ZONE_CONTROL_ENTRY_INCLUDE: &str = "$INCLUDE";
pub const ZONE_CONTROL_ENTRY_INCLUDE_LEN: usize = ZONE_CONTROL_ENTRY_INCLUDE.len();

pub const ZONE_CONTROL_ENTRY_ORIGIN: &str = "$ORIGIN";
pub const ZONE_CONTROL_ENTRY_ORIGIN_LEN: usize = ZONE_CONTROL_ENTRY_ORIGIN.len();
