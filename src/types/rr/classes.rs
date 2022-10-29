use crate::packing::{UnpackBuffer, UnpackBufferResult, Unpackable};

/// [`Class`] describes resource record class codes.
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4
#[derive(Debug, Clone)]
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

    /// If we receive an invalid RR class, we fallback to this "class" after which we can terminate the connection
    BOGUS,
}

impl Default for Class {
    fn default() -> Self {
        Self::IN
    }
}

impl Unpackable for Class {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let class = match u16::unpack(buf) {
            Ok(class) => class,
            Err(err) => return Err(err),
        };

        Ok(Class::from(class))
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match self {
            Class::IN => String::from("IN"),
            Class::CS => String::from("CS"),
            Class::CH => String::from("CH"),
            Class::HS => String::from("HS"),
            Class::ANY => String::from("ANY"),
            Class::BOGUS => String::from("BOGUS"),
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
            _ => Self::BOGUS,
        };
    }
}
