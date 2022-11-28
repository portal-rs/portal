use std::fmt::Display;

use crate::{
    constants,
    error::ProtocolError,
    packing::{
        PackBuffer, PackBufferResult, Packable, UnpackBuffer, UnpackBufferResult, Unpackable,
    },
};

enum NameParseState {
    LabelLenOrPointer,
    Pointer,
    Label,
    Root,
}

impl Default for NameParseState {
    fn default() -> Self {
        return Self::LabelLenOrPointer;
    }
}

#[derive(Debug, Clone)]
pub struct Name {
    labels: Vec<Label>,
}

impl Default for Name {
    fn default() -> Self {
        Self { labels: Vec::new() }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_dotted_string())
    }
}

impl Unpackable for Name {
    fn unpack(buf: &mut UnpackBuffer) -> UnpackBufferResult<Self> {
        let mut state = NameParseState::default();
        let mut name = Name::default();

        // If we immediatly encounter a null byte, the name is root "."
        // This can be simplified when if let chains are stabilized. The
        // current solution is fugly ngl...
        //
        // if let Some(b) = buf.peek() && b == 0 {
        //     buf.pop();
        //     return Ok(name);
        // }
        match buf.peek() {
            Some(b) if b == 0 => {
                buf.pop()?;
                return Ok(name);
            }
            _ => {}
        }

        loop {
            state = match state {
                NameParseState::LabelLenOrPointer => match buf.peek() {
                    // We reached the end of the buf or encountered the
                    // terminating null byte
                    Some(0) | None => NameParseState::Root,

                    // We encountered a compression pointer, follow it
                    Some(b) if b & 0xC0 == 0xC0 => NameParseState::Pointer,

                    // We encountered a normal label length byte, read
                    // characters until label len
                    Some(b) if b & 0xC0 == 0x0 => NameParseState::Label,

                    // A byte which shouldn't be here
                    Some(b) => return Err(ProtocolError::InvalidLabelLenOrPointer(b)),
                },
                NameParseState::Pointer => {
                    // Read a u16 which starts with 11 (0xC0) and apply the bit
                    // mask to extract the actual compression pointer location
                    let pointer_location = match u16::unpack(buf) {
                        Ok(b) => (b & constants::dns::COMPRESSION_POINTER_MASK) as usize,
                        Err(err) => return Err(err),
                    };

                    // Ensure we jump to a location which comes before the
                    // current offset
                    if pointer_location > buf.offset() {
                        return Err(ProtocolError::InvalidPointerLocation);
                    }

                    // Jump to the pointer location by updating the underlying
                    // buffer
                    buf.jump_to(pointer_location)?;
                    NameParseState::LabelLenOrPointer
                }
                NameParseState::Label => {
                    // Read the label based on the label length byte. This
                    // returns an error if the label length exceeds the maximum
                    // domain name label length of 63
                    let label = match buf.unpack_character_string(constants::dns::MAX_LABEL_LENGTH)
                    {
                        Ok(label) => label,
                        Err(_) => return Err(ProtocolError::DomainNameLabelTooLong),
                    };

                    // Add the label to the domain name. This returns an error
                    // if the domain name length exceeds the maximum domain
                    // name length of 255
                    if let Err(_) = name.add_label(label.into()) {
                        return Err(ProtocolError::DomainNameTooLong);
                    }

                    NameParseState::LabelLenOrPointer
                }
                NameParseState::Root => {
                    // We followed one ore more compression pointers and now
                    // need to jump back to continue resolving the pointer
                    // chain
                    if buf.followed_pointers() {
                        buf.jump_back();

                        // state = NameParseState::LabelLenOrPointer;
                        // continue;

                        // Should we break here? Can there be multiple nested
                        // compression pointers?
                        break;
                    }

                    // We reached the terminating null byte. Remove it from
                    // the buffer and break out of the loop
                    buf.pop()?;
                    break;
                }
            }
        }

        Ok(name)
    }
}

impl Packable for Name {
    fn pack(&self, buf: &mut PackBuffer) -> PackBufferResult {
        let buffer_len_start = buf.len();

        // TODO (Techassi): This does NOT handle compression. Add it
        for label in self.iter() {
            if label.len() > constants::dns::MAX_LABEL_LENGTH.into() {
                return Err(ProtocolError::DomainNameLabelTooLong);
            }

            buf.push(label.len() as u8);
            buf.pack_vec(&mut label.bytes())?;
        }

        // Terminating null byte
        buf.push(0);

        if buf.len() - buffer_len_start > constants::dns::MAX_DOMAIN_LENGTH.into() {
            return Err(ProtocolError::DomainNameTooLong);
        }

        Ok(())
    }
}

impl TryFrom<String> for Name {
    type Error = ProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut name = Self::default();

        // If root, return Name::default()
        if value == "." {
            return Ok(name);
        }

        let parts = value.split('.');
        for part in parts {
            if part != "" {
                name.add_label(Label::from(part.as_bytes()))?;
            }
        }

        return Ok(name);
    }
}

impl TryFrom<&str> for Name {
    type Error = ProtocolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl Name {
    /// Return an [`Iterator`] over the labels in the domain name.
    pub fn iter(&self) -> NameIterator<'_> {
        NameIterator {
            name: self,
            index: 0,
        }
    }

    pub fn add_label(&mut self, label: Label) -> Result<(), ProtocolError> {
        if self.len() + label.0.len() > constants::dns::MAX_DOMAIN_LENGTH.into() {
            return Err(ProtocolError::DomainNameTooLong);
        }

        self.labels.push(label);
        Ok(())
    }

    /// Return the number of labels without the root "." label.
    pub fn num_labels(&self) -> usize {
        return self.labels.len();
    }

    /// Return the number of labels with the root "." label.
    pub fn num_labels_root(&self) -> usize {
        return self.labels.len() + 1;
    }

    pub fn len(&self) -> usize {
        let dots = self.labels.len();

        let mut labels = 0;
        self.labels.iter().for_each(|l| labels += l.0.len());

        return dots + labels;
    }

    pub fn to_dotted_string(&self) -> String {
        self.labels
            .iter()
            .map(|l| {
                let mut label = l.to_string();
                label.push('.');
                label
            })
            .collect()
    }
}

pub struct NameIterator<'a> {
    name: &'a Name,
    index: usize,
}

impl<'a> Iterator for NameIterator<'a> {
    type Item = &'a Label;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.name.len() {
            return None;
        }

        let current_label = self.name.labels.get(self.index);
        self.index += 1;

        return current_label;
    }
}

impl<'a> ExactSizeIterator for NameIterator<'a> {}

#[derive(Debug, Clone)]
pub struct Label(Vec<u8>);

impl From<&[u8]> for Label {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

impl ToString for Label {
    fn to_string(&self) -> String {
        match String::from_utf8(self.0.clone()) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }
}

impl Label {
    // TODO (Techassi): This idealy should not clone, but we need to introduce
    // lifetimes across Label, Name and types using Name, e.g. Question
    pub fn bytes(&self) -> Vec<u8> {
        return self.0.clone();
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }
}
