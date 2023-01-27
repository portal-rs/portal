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
        Self::LabelLenOrPointer
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Name {
    labels: Vec<Label>,
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_dotted_string())
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
                    let bytes = match buf.unpack_character_string(constants::dns::MAX_LABEL_LENGTH)
                    {
                        Ok(bytes) => bytes,
                        Err(_) => return Err(ProtocolError::DomainNameLabelTooLong),
                    };

                    // Add the label to the domain name. This returns an error
                    // if the domain name length exceeds the maximum domain
                    // name length of 255
                    name.add_label(bytes.try_into()?)?;

                    NameParseState::LabelLenOrPointer
                }
                NameParseState::Root => {
                    // We followed one ore more compression pointers and now
                    // need to jump back to continue resolving the pointer
                    // chain
                    if buf.iter_back() {
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

        if !value.is_ascii() {
            return Err(ProtocolError::InvalidOmainNameLabelByte);
        }

        let parts = value.split('.');
        for part in parts {
            if !part.is_empty() {
                name.add_label(part.as_bytes().try_into()?)?;
            }
        }

        Ok(name)
    }
}

impl TryFrom<&str> for Name {
    type Error = ProtocolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string().to_lowercase())
    }
}

impl TryFrom<&[Label]> for Name {
    type Error = ProtocolError;

    fn try_from(value: &[Label]) -> Result<Self, Self::Error> {
        let mut name = Name::default();

        for label in value {
            name.add_label(label.clone())?
        }

        Ok(name)
    }
}

impl Name {
    /// Return an [`Iterator`] over the labels in the domain name.
    ///
    /// ### Example
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let n = Name::try_from("www.example.com").unwrap();
    /// let s = n.iter()
    ///     .map(|l| {
    ///         let mut label = l.to_string();
    ///         label.push('.');
    ///         label
    ///     })
    ///     .collect::<String>();
    ///
    /// assert_eq!(n.as_dotted_string(), s);
    /// ```
    ///
    /// ### Example (Reversed)
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let n = Name::try_from("www.example.com").unwrap();
    /// let s = n.iter().rev().map(|l| l.to_string()).collect::<String>();
    ///
    /// println!("{}", s);
    /// ```
    pub fn iter(&self) -> NameIterator<'_> {
        NameIterator {
            name: self,
            index_back: self.num_labels(),
            index: 0,
        }
    }

    /// Adds a label to the domain. This validates the following two conditions:
    ///
    /// - The total length of the domain does not exceed the maximum allowed
    ///   length [`MAX_DOMAIN_LENGTH`][constants::dns::MAX_DOMAIN_LENGTH]
    /// - The length of the label does not exceed the maximum allowed label
    ///   length [`MAX_LABEL_LENGTH`][constants::dns::MAX_LABEL_LENGTH]
    ///
    /// ### Example
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let mut n = Name::try_from("www.example").unwrap();
    /// n.add_label("com".try_into()?)?;
    ///
    /// assert_eq!(n.as_dotted_string(), String::from("www.example.com."));
    /// # Ok::<(), portal::errors::ProtocolError>(())
    /// ```
    pub fn add_label(&mut self, label: Label) -> Result<(), ProtocolError> {
        if self.len() + label.0.len() > constants::dns::MAX_DOMAIN_LENGTH.into() {
            return Err(ProtocolError::DomainNameTooLong);
        }

        if label.0.len() > constants::dns::MAX_LABEL_LENGTH.into() {
            return Err(ProtocolError::DomainNameLabelTooLong);
        }

        self.labels.push(label);
        Ok(())
    }

    /// Returns a reference to the underlying vector of labels with default
    /// ordering.
    pub fn labels(&self) -> &Vec<Label> {
        self.labels.as_ref()
    }

    /// Returns the domain name as fragments. For example `www.example.com`
    /// returns `vec![com, example.com, www.example.com]`. This function
    /// is memory and CPU heavy as we need to copy and alloocate quite a lot.
    /// Use with caution!
    pub fn fragments(&self) -> Vec<Name> {
        let mut fragments: Vec<Name> = Vec::new();
        let labels = self.labels();

        for i in 0..labels.len() {
            let parts = &labels[labels.len() - i - 1..labels.len()];
            let name = Self::try_from(parts).unwrap_or_default();
            fragments.push(name)
        }

        fragments
    }

    // NOTE (Techassi): We could think about storing the reverse name alongside
    // the rest of the data. This would increase the memory footprint, but we
    // could avoid cloning the data when calling this method.

    /// Returns the underlying vector of labels in reverse order. This is
    /// helpful when traversing a DNS tree (e.g. cache).
    pub fn labels_rev(&self) -> Vec<Label> {
        self.iter().rev().cloned().collect()
    }

    /// Returns the number of labels without the root "." label.
    ///
    /// ### Example
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let n = Name::try_from("www.example.com").unwrap();
    /// assert_eq!(n.num_labels(), 3);
    ///
    /// let n = Name::try_from("www.example.com.").unwrap();
    /// assert_eq!(n.num_labels(), 3);
    /// ```
    pub fn num_labels(&self) -> usize {
        self.labels.len()
    }

    /// Return the number of labels with the root "." label.
    ///
    /// ### Example
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let n = Name::try_from("www.example.com").unwrap();
    /// assert_eq!(n.num_labels_root(), 4);
    ///
    /// let n = Name::try_from("www.example.com.").unwrap();
    /// assert_eq!(n.num_labels_root(), 4);
    /// ```
    pub fn num_labels_root(&self) -> usize {
        self.labels.len() + 1
    }

    /// Returns the total length (in bytes) required in the wire format. This
    /// includes the length octets between labels and the terminating null byte
    /// (or root ".").
    ///
    /// ### Example
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let n = Name::try_from("www.example.com").unwrap();
    /// assert_eq!(n.len(), 17);
    /// ```
    pub fn len(&self) -> usize {
        let dots = self.num_labels_root();

        let mut labels = 0;
        self.iter().for_each(|l| labels += l.0.len());

        dots + labels
    }

    /// Returns if the domain name only consists if the root "." label.
    ///
    /// ### Example
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let n = Name::try_from(".").unwrap();
    /// assert!(n.is_root());
    ///
    /// let n = Name::try_from("www.example.com").unwrap();
    /// assert!(!n.is_root());
    /// ```
    pub fn is_root(&self) -> bool {
        self.labels.is_empty()
    }

    /// Returns the domain as a dotted string.
    ///
    /// ### Example
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let n = Name::try_from("www.example.com").unwrap();
    /// assert_eq!(n.as_dotted_string(), String::from("www.example.com."))
    /// ```
    pub fn as_dotted_string(&self) -> String {
        if self.is_root() {
            return String::from(".");
        }

        self.iter()
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
    index_back: usize,
    index: usize,
}

impl<'a> Iterator for NameIterator<'a> {
    type Item = &'a Label;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.name.num_labels() {
            return None;
        }

        let current_label = self.name.labels.get(self.index);
        self.index += 1;

        current_label
    }
}

impl<'a> DoubleEndedIterator for NameIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index_back == 0 {
            return None;
        }

        let current_label = self.name.labels.get(self.index_back - 1);
        self.index_back -= 1;

        current_label
    }
}

impl<'a> ExactSizeIterator for NameIterator<'a> {}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Label(Vec<u8>);

impl TryFrom<&[u8]> for Label {
    type Error = ProtocolError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() > constants::dns::MAX_LABEL_LENGTH.into() {
            return Err(ProtocolError::DomainNameLabelTooLong);
        }

        let bytes = bytes
            .iter()
            .cloned()
            .map_while(validate_domain_name_byte)
            .collect();

        Ok(Self(bytes))
    }
}

// TODO (Techassi): Check if the str contains any non allowed chars
impl TryFrom<&str> for Label {
    type Error = ProtocolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_lowercase().as_bytes())
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
    pub fn new() -> Self {
        Self::default()
    }

    // TODO (Techassi): This idealy should not clone, but we need to introduce
    // lifetimes across Label, Name and types using Name, e.g. Question
    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
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
