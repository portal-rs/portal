use std::{fmt::Display, str::FromStr};

use binbuf::{
    read::{ReadBuffer, ReadError, Readable},
    write::{WriteBuffer, Writeable},
    Endianness,
};
use serde::Serialize;
use snafu::{ensure, ResultExt, Snafu};

use crate::constants::{COMP_PTR, COMP_PTR_MASK, MAX_DOMAIN_LENGTH, MAX_LABEL_LENGTH};

mod label;
pub use label::*;

#[derive(Debug, Default)]
enum NameParseState {
    #[default]
    LabelLenOrPointer,
    Pointer,
    Label,
    Root,
}

#[derive(Debug, Snafu)]
pub enum NameError {
    #[snafu(display("invalid domain name label length or compression pointer ({ptr})"))]
    InvalidLabelLenOrPointer { ptr: u8 },

    #[snafu(display("failed to read label"))]
    ReadLabel { source: LabelError },

    #[snafu(display("failed to write label, expected < {} max bytes", MAX_LABEL_LENGTH))]
    WriteLabelTooLong,

    #[snafu(display("failed to read terminating null byte"))]
    ReadTerminatingNullByte { source: ReadError },

    #[snafu(display("invalid compression pointer location"))]
    InvalidPointerLocation,

    #[snafu(display("failed to read compression pointer"))]
    ReadPointer { source: ReadError },

    #[snafu(display("domain name to long (< {})", MAX_DOMAIN_LENGTH))]
    DomainNameTooLong,
}

#[derive(Debug, Snafu)]
pub enum NameParseError {
    #[snafu(display("input contains non-ASCII characters"))]
    NonAscii,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Name {
    labels: Vec<Label>,
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_dotted_string().as_str())
    }
}

impl Readable for Name {
    type Error = NameError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let mut state = NameParseState::default();
        let mut name = Name::default();

        // If we immediately encounter a null byte, the name is root "."
        // This can be simplified when if let chains are stabilized. The
        // current solution is a little more verbose than I like it
        // to be...
        //
        // if let Some(b) = buf.peek() && b == 0 {
        //     buf.pop();
        //     return Ok(name);
        // }
        if buf.peek().filter(|b| *b == 0).is_some() {
            // We just made sure there is at least once byte left in the
            // buffer. So it is save to unwrap here.
            buf.pop().unwrap();
            return Ok(name);
        }

        loop {
            state = match state {
                NameParseState::LabelLenOrPointer => match buf.peek() {
                    // We reached the end of the buf or encountered the
                    // terminating null byte
                    Some(0) | None => NameParseState::Root,

                    // We encountered a compression pointer, follow it
                    Some(b) if b & COMP_PTR == COMP_PTR => NameParseState::Pointer,

                    // We encountered a normal label length byte, read
                    // characters until label len
                    Some(b) if b & COMP_PTR == 0x0 => NameParseState::Label,

                    // A byte which shouldn't be here
                    Some(b) => return InvalidLabelLenOrPointerSnafu { ptr: b }.fail(),
                },
                NameParseState::Pointer => {
                    // Read a u16 which starts with 11 (0xC0) and apply the bit
                    // mask to extract the actual compression pointer location
                    let raw_pointer = u16::read::<E>(buf).context(ReadPointerSnafu)?;
                    let pointer = (raw_pointer & COMP_PTR_MASK) as usize;

                    // Ensure we jump to a location which comes before the
                    // current offset
                    ensure!(pointer <= buf.offset(), InvalidPointerLocationSnafu);

                    // Jump to the pointer location by updating the underlying
                    // buffer. Above we made sure the location is valid and thus
                    // it is save to unwrap.
                    buf.jump_to(pointer).unwrap();
                    NameParseState::LabelLenOrPointer
                }
                NameParseState::Label => {
                    // Read the label based on the label length byte. This
                    // returns an error if the label length exceeds the maximum
                    // domain name label length of 63
                    let label = Label::read::<E>(buf).context(ReadLabelSnafu)?;

                    // Add the label to the domain name. This returns an error
                    // if the domain name length exceeds the maximum domain
                    // name length of 255
                    name.add_label(label)?;

                    NameParseState::LabelLenOrPointer
                }
                NameParseState::Root => {
                    // We followed one ore more compression pointers and now
                    // need to jump back to continue resolving the pointer
                    // chain
                    if buf.jump_reset() {
                        break;
                    }

                    // We reached the terminating null byte. Remove it from
                    // the buffer and break out of the loop
                    buf.pop().context(ReadTerminatingNullByteSnafu)?;
                    break;
                }
            }
        }

        Ok(name)
    }
}

impl Writeable for Name {
    type Error = NameError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let buffer_len_start = buf.len();
        let mut n = 0;

        // TODO (Techassi): This does NOT handle compression. Add it
        for label in self.iter() {
            let label_len = label.len();
            ensure!(label_len <= MAX_LABEL_LENGTH.into(), WriteLabelTooLongSnafu);

            buf.push(label_len as u8);
            n += buf.write(&mut label.bytes());
        }

        // Terminating null byte
        buf.push(0);
        n += 1;

        if buf.len() - buffer_len_start > MAX_DOMAIN_LENGTH.into() {
            return Err(NameError::DomainNameTooLong);
        }

        Ok(n)
    }
}

impl FromStr for Name {
    type Err = NameParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        ensure!(input.is_ascii(), NonAsciiSnafu);
        let mut name = Self::default();

        // If root, return Name::default()
        if input == "." {
            return Ok(name);
        }

        let parts = input.split('.');
        for part in parts {
            if !part.is_empty() {
                name.add_label(part.as_bytes().try_into()?)?;
            }
        }

        Ok(name)
    }
}

impl TryFrom<&[Label]> for Name {
    type Error = NameError;

    fn try_from(value: &[Label]) -> Result<Self, Self::Error> {
        let mut name = Name::default();

        for label in value {
            name.add_label(label.clone())?
        }

        Ok(name)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_dotted_string())
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
    /// # Ok::<(), portal::types::dns::NameError>(())
    /// ```
    pub fn add_label(&mut self, label: Label) -> Result<(), NameError> {
        ensure!(
            self.size() + label.0.len() <= MAX_LABEL_LENGTH.into(),
            DomainNameTooLongSnafu
        );

        ensure!(
            label.0.len() <= MAX_LABEL_LENGTH.into(),
            WriteLabelTooLongSnafu
        );

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
    /// is memory and CPU heavy as we need to copy and allocate quite a lot.
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

    /// Returns the total size (in bytes) required in the wire format. This
    /// includes the length octets between labels and the terminating null byte
    /// (or root ".").
    ///
    /// ### Example
    ///
    /// ```
    /// use portal::types::dns::Name;
    ///
    /// let n = Name::try_from("www.example.com").unwrap();
    /// assert_eq!(n.size(), 17);
    /// ```
    pub fn size(&self) -> usize {
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
