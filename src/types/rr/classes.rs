use std::fmt::Display;

use binbuf::prelude::*;
use serde::Serialize;

use crate::errors::ProtocolError;

/// [`Class`] describes resource record class codes.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4
#[derive(Debug, Clone, Copy, Serialize)]
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
    /// This can be the case when we deal with OPT records. ENDS uses this
    /// field to indicate the sender's UDP payload size instead of the class.
    /// To be able to use the value, we add the `u16` to this variant.
    UNKNOWN(u16),
}

impl Default for Class {
    fn default() -> Self {
        Self::IN
    }
}

impl Readable for Class {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let class = u16::read::<E>(buf)?;
        Ok(Class::from(class))
    }
}

impl Writeable for Class {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let class: u16 = self.into();
        class.write::<E>(buf)
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
            Class::UNKNOWN(c) => write!(f, "UNKNOWN({c})"),
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
        match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            255 => Self::ANY,
            _ => Self::UNKNOWN(value),
        }
    }
}

impl From<Class> for u16 {
    fn from(value: Class) -> Self {
        match value {
            Class::IN => 1,
            Class::CS => 2,
            Class::CH => 3,
            Class::HS => 4,
            Class::ANY => 255,
            Class::UNKNOWN(v) => v,
        }
    }
}

impl From<&Class> for u16 {
    fn from(value: &Class) -> Self {
        Self::from(*value)
    }
}
