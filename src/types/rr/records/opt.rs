/// See https://datatracker.ietf.org/doc/html/rfc6891#section-6.1
/// Many OPT header fields have a special usa case:
/// - Class: UDP payload size. RFC 6891 romoved the 512 byte size limit.
///   This field can set the maximum size in bytes (e.g. 4096)
/// - TTL: This 32 bit field is split up in:
///   - 1 octet extended RCODEs
///   - 1 octet EDNS version
///   - DO bit
///   - 15 reserved bits
pub struct OPT {}
