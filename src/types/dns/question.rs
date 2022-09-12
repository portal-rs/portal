use crate::types::rr::{Class, Type};

/// [`Question`] describes a DNS question. The RFC allows multiple questions per message, but most DNS servers only
/// accpet one and multiple questions often result in errors.
///
/// ### Further information
///
/// See https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.2
#[derive(Debug, Clone)]
pub struct Question {
    pub name: String,
    pub typ: Type,
    pub class: Class,
}

impl Default for Question {
    fn default() -> Self {
        Self {
            name: Default::default(),
            typ: Type::NONE,
            class: Class::IN,
        }
    }
}
