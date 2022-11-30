use std::{fs, path::PathBuf, str::FromStr};

use crate::{
    constants,
    types::{
        dns::Name,
        rr::{Class, Record, Type},
    },
};

mod error;

pub use error::*;

pub enum ZoneParseState<'a> {
    NewLine,
    Entry(&'a str),
    Origin(&'a str),
    Include(&'a str),
    Record(&'a str),
    ClassAndTTL(RecordParseState<'a>),
    ClassOrTTL(RecordParseState<'a>),
    RecordRest(RecordParseState<'a>),
}

#[derive(Debug)]
pub struct RecordParseState<'a> {
    class: Option<Class>,
    parts: Vec<&'a str>,
    ttl: Option<u32>,
    line: &'a str,
    name: Name,
}

impl<'a> Default for ZoneParseState<'a> {
    fn default() -> Self {
        Self::NewLine
    }
}

pub struct Zone {
    records: Vec<Record>,
}

impl Default for Zone {
    fn default() -> Self {
        Self {
            records: Default::default(),
        }
    }
}

impl FromStr for Zone {
    type Err = ZoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut zone = Zone::default();

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

                    // Split the line at ";". When there is a comment we can
                    // savely ignore everything after the semicolon. If there
                    // is not semicolon (parts len = 1) we can just use the
                    // line contents as is.
                    let parts: Vec<&str> = line.split(';').collect();
                    let line = parts[0];

                    // At this point we have to parse space / tab separated
                    // items.
                    let parts: Vec<&str> =
                        line.split([' ', '\t']).filter(|p| !p.is_empty()).collect();

                    // Now we need to parse a domain name.
                    let name = match Name::try_from(parts[0]) {
                        Ok(name) => name,
                        Err(err) => return Err(ZoneError::ParseError(err.to_string())),
                    };

                    // For what ever bonker reasons the RFC 1035 states there
                    // can be two forms how the RR is formatted. Either the
                    // class or TTL comes first. AND they can BOTH be optional.
                    // If the length of parts is only 3, we assume both class
                    // and TTL is missing, skipping the parsing of those
                    // completely.
                    let parse_state = RecordParseState {
                        parts: parts[1..].to_vec(),
                        class: None,
                        ttl: None,
                        line,
                        name,
                    };

                    if parts.len() == 5 {
                        state = ZoneParseState::ClassAndTTL(parse_state);
                        continue;
                    }

                    if parts.len() == 4 {
                        state = ZoneParseState::ClassOrTTL(parse_state);
                        continue;
                    }

                    ZoneParseState::RecordRest(parse_state)
                }
                ZoneParseState::ClassAndTTL(parse_state) => {
                    // So we first try to parse the
                    // next part (1) as a class. If this fails, we assume the
                    // next part (2) HAS to be a class, otherwise this is
                    // invalid input
                    let mut parse_ttl_at = 1;
                    let class = match Class::try_from(parse_state.parts[0]) {
                        Ok(class) => class,
                        Err(_) => match Class::try_from(parse_state.parts[1]) {
                            Ok(class) => {
                                parse_ttl_at = 0;
                                class
                            }
                            Err(err) => return Err(ZoneError::ParseError(err.to_string())),
                        },
                    };

                    // Now parse the TTL at the correct index, which is defined
                    // by the parsing step above.
                    let ttl = match u32::from_str(parse_state.parts[parse_ttl_at]) {
                        Ok(ttl) => ttl,
                        Err(err) => return Err(ZoneError::ParseError(err.to_string())),
                    };

                    ZoneParseState::RecordRest(RecordParseState {
                        parts: parse_state.parts[2..].to_vec(),
                        line: parse_state.line,
                        name: parse_state.name,
                        class: Some(class),
                        ttl: Some(ttl),
                    })
                }
                ZoneParseState::ClassOrTTL(mut parse_state) => {
                    match Class::try_from(parse_state.parts[0]) {
                        Ok(class) => {
                            parse_state.class = Some(class);
                            parse_state.parts = parse_state.parts[1..].to_vec();
                            state = ZoneParseState::RecordRest(parse_state);
                            continue;
                        }
                        Err(_) => match u32::from_str(parse_state.parts[0]) {
                            Ok(ttl) => {
                                parse_state.ttl = Some(ttl);
                                parse_state.parts = parse_state.parts[1..].to_vec();
                                state = ZoneParseState::RecordRest(parse_state);
                                continue;
                            }
                            Err(err) => return Err(ZoneError::ParseError(err.to_string())),
                        },
                    };
                }
                ZoneParseState::RecordRest(state) => {
                    // Now we are back in save territory. The rest of the RR
                    // has the same format across the board. We now need to
                    // parse the RR type.
                    let ty = match Type::try_from(state.parts[0]) {
                        Ok(ty) => ty,
                        Err(err) => return Err(ZoneError::ParseError(err.to_string())),
                    };

                    // The last piece of input is the RDATA.

                    println!("{} {:?} {:?} {}", state.name, state.class, state.ttl, ty);

                    // Now add the record to the zone

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
