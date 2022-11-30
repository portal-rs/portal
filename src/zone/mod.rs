use std::{fs, path::PathBuf, str::FromStr};

use crate::{constants, types::rr::Record};

mod error;

pub use error::*;

pub enum ZoneParseState<'a> {
    NewLine,
    Entry(&'a str),
    Origin(&'a str),
    Include(&'a str),
    Record(&'a str),
}

impl<'a> Default for ZoneParseState<'a> {
    fn default() -> Self {
        Self::NewLine
    }
}

pub struct Zone {
    records: Vec<Record>,
}

impl FromStr for Zone {
    type Err = ZoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = ZoneParseState::default();
        let mut lines = s.lines();

        loop {
            state = match state {
                ZoneParseState::NewLine => match lines.next() {
                    // There are still lines left, continue
                    Some(line) => {
                        // If the line is completely empty, skip
                        if line.is_empty() {
                            continue;
                        }

                        // Lines starting with a semicolon are comments and
                        // can be ignored
                        if line.starts_with(';') {
                            continue;
                        }

                        // If there is content, it has to be an entry
                        ZoneParseState::Entry(line)
                    }
                    // We reached EOF, break
                    None => break,
                },
                ZoneParseState::Entry(line) => {
                    // We encountered an $ORIGIN control entry. $ORIGIN is
                    // followed by a domain name, and resets the current origin
                    // for relative domain names to the stated name.
                    if line.starts_with(constants::zone::ZONE_CONTROL_ENTRY_ORIGIN) {
                        let (_, rest) =
                            line.split_at(constants::zone::ZONE_CONTROL_ENTRY_ORIGIN_LEN);

                        state = ZoneParseState::Origin(rest);
                        continue;
                    }

                    // We encountered an $INCLUDE control entry. $INCLUDE
                    // inserts the named file into the current file, and may
                    // optionally specify a domain name that sets the relative
                    // domain name origin for the included file.
                    if line.starts_with(constants::zone::ZONE_CONTROL_ENTRY_INCLUDE) {
                        let (_, rest) =
                            line.split_at(constants::zone::ZONE_CONTROL_ENTRY_INCLUDE_LEN);

                        state = ZoneParseState::Include(rest);
                        continue;
                    }

                    // We encountered a normal record entry
                    ZoneParseState::Record(line)
                }
                ZoneParseState::Origin(line) => {
                    let line = line.trim();
                    let parts = line.split(';');

                    ZoneParseState::NewLine
                }
                ZoneParseState::Include(_) => todo!(),
                ZoneParseState::Record(line) => {
                    let line = line.trim();
                    println!("{}", line);
                    ZoneParseState::NewLine
                }
            }
        }

        Ok(Zone {
            records: Vec::new(),
        })
    }
}

impl Zone {
    pub fn from_file(path: PathBuf) -> Result<Self, ZoneError> {
        let b = match fs::read_to_string(path) {
            Ok(b) => b,
            Err(err) => return Err(ZoneError::IO(err)),
        };

        let zone: Zone = b.parse()?;
        Ok(zone)
    }

    pub fn to_file(&self, path: PathBuf) -> Result<(), ZoneError> {
        Ok(())
    }
}
