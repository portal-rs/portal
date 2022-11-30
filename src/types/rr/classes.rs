use std::fmt::Display;

use crate::{
    errors::ProtocolError,
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
};

/// [`Class`] describes resource record class codes.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4
#[derive(Debug, Clone, Copy)]
pub enum Class {
    /// The Internet
    IN,

    /// The CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS,

    /// The CHAOS class
    CH,

    /// Hesiod [Dyer 87]
    HS,

    // QClasses are a superset of classes and should only be allowed in questions
    /// Any class (*)
    ANY,

    /// If we receive an unknown RR class, we default back to this "class".
    /// This can be the case when wie deal with OPT records. ENDS uses this
    /// field to indicate the sender's UDP payload size instead of the class.
    /// To be able to use the value, we add the `u16` to this variant.
    UNKNOWN(u16),
}

impl Default for Class {
    fn default() -> Self {
        Self::IN
    }
}

impl Unpackable for Class {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let class = u16::unpack(buf)?;
        Ok(Class::from(class))
    }
}

impl Packable for Class {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let class: u16 = self.into();
        class.pack(buf)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::IN => write!(f, "IN"),
            Class::CS => write!(f, "CS"),
            Class::CH => write!(f, "CH"),
            Class::HS => write!(f, "HS"),
            Class::ANY => write!(f, "ANY"),
            Class::UNKNOWN(c) => write!(f, "UNKNOWN({})", c),
        }
    }
}

impl TryFrom<&str> for Class {
    type Error = ProtocolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "IN" => Ok(Self::IN),
            "CS" => Ok(Self::CS),
            "CH" => Ok(Self::CH),
            "HS" => Ok(Self::HS),
            "ANY" => Ok(Self::ANY),
            _ => Err(ProtocolError::ClassParseError),
        }
    }
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        return match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            255 => Self::ANY,
            _ => Self::UNKNOWN(value),
        };
    }
}

impl Into<u16> for Class {
    fn into(self) -> u16 {
        match self {
            Class::IN => 1,
            Class::CS => 2,
            Class::CH => 3,
            Class::HS => 4,
            Class::ANY => 255,
            Class::UNKNOWN(v) => v,
        }
    }
}

impl Into<u16> for &Class {
    fn into(self) -> u16 {
        (*self).into()
    }
}
