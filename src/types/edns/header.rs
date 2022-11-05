use crate::types::{dns::Name, rr::RHeader};

#[derive(Debug)]
pub struct EdnsHeader {
    /// This will always be empty (root).
    name: Name,

    /// The sender's UDP payload size.
    ///
    /// ### Notes
    ///
    /// It describes the number of octets of the largest UDP payload that can
    /// be reassembled and delivered in the sender's network stack. Note that
    /// path MTU, with or without fragmentation, may be smaller than this. See
    /// [RFC 2671](https://www.rfc-editor.org/rfc/rfc2671#section-4.5).
    sender_payload_size: u16,

    /// The upper 8 bits of extended 12-bit RCODE.
    ///
    /// ### Notes
    ///
    /// Note that EXTENDED-RCODE value "0" indicates that an unextended RCODE
    /// is in use (values "0" through "15").
    /// See [RFC 2671](https://www.rfc-editor.org/rfc/rfc2671#section-4.6).
    upper_ext_rcode: u8,

    /// EDNS version. This helps to indicate which level is supported on client
    /// side.
    ///
    /// ### Notes
    ///
    /// Indicates the implementation level of whoever sets it. Full conformance
    /// with this specification is indicated by version "0". Requestors are
    /// encouraged to set this to the lowest implemented level capable of
    /// expressing a transaction, to minimize the responder and network load of
    /// discovering the greatest common implementation level between requestor
    /// and responder. A requestor's version numbering strategy should ideally
    /// be a run time configuration option.
    ///
    /// If a responder does not implement the VERSION level of the request,
    /// then it answers with RCODE=BADVERS. All responses will be limited in
    /// format to the VERSION level of the request, but the VERSION of each
    /// response will be the highest implementation level of the responder.
    /// In this way a requestor will learn the implementation level of a
    /// responder as a side effect of every response, including error
    /// responses, including RCODE=BADVERS.
    /// See [RFC 2671](https://www.rfc-editor.org/rfc/rfc2671#section-4.6)
    version: u8,

    /// Zero padding
    ///
    /// ### Notes
    ///
    /// Set to zero by senders and ignored by receivers, unless modified in a
    /// subsequent specification.
    zero: u16,
}

impl From<&RHeader> for EdnsHeader {
    fn from(rheader: &RHeader) -> Self {
        Self {
            name: rheader.name.clone(),
            sender_payload_size: rheader.class.into(),
            upper_ext_rcode: (rheader.ttl & 0xFF) as u8,
            version: ((rheader.ttl << 8) & 0xFF) as u8,
            zero: (rheader.ttl << 16) as u16,
        }
    }
}
