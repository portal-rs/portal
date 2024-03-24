use binbuf::{
    read::{ReadBuffer, ReadError},
    Endianness, Readable,
};
use snafu::{ResultExt, Snafu};

use crate::constants::MAX_LABEL_LENGTH;

#[derive(Debug, Snafu)]
pub enum LabelError {
    TooLong { source: ReadError },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Label(pub Vec<u8>);

impl Readable for Label {
    type Error = LabelError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        // TODO (Techassi): Check if the str contains any non allowed chars
        let bytes = buf
            .read_char_string(Some(MAX_LABEL_LENGTH))
            .context(TooLongSnafu)?;

        Ok(Self(bytes.to_vec()))
    }
}

// impl TryFrom<&[u8]> for Label {
//     type Error = LabelError;

//     fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
//         if bytes.len() > MAX_LABEL_LENGTH.into() {
//             return Err(LabelError::TooLong);
//         }

//         let bytes = bytes
//             .iter()
//             .cloned()
//             .map_while(validate_domain_name_byte)
//             .collect();

//         Ok(Self(bytes))
//     }
// }

// impl FromStr for Label {
//     type Err = LabelError;

//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         // TODO (Techassi): Check if the str contains any non allowed chars
//         Self::try_from(input.as_bytes())
//     }
// }

impl ToString for Label {
    fn to_string(&self) -> String {
        match String::from_utf8(self.0.clone()) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }
}

impl Label {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO (Techassi): This ideally should not clone, but we need to introduce
    // lifetimes across Label, Name and types using Name, e.g. Question
    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn validate_domain_name_byte(byte: u8) -> Option<u8> {
    match byte {
        45 => Some(byte),           // Hyphen
        48..=57 => Some(byte),      // Digits
        65..=90 => Some(byte + 32), // Uppercase letters
        97..=122 => Some(byte),     // Lowercase letters
        _ => None,
    }
}
