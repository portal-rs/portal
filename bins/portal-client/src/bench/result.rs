use std::time::Duration;

use portal::types::{dns::Name, rr::Type};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BenchResult {
    pub error: Option<String>,
    pub duration: Option<u128>,
    pub number: usize,
    pub name: Name,

    #[serde(rename = "type")]
    pub ty: Type,
}

impl BenchResult {
    pub fn success(number: usize, name: &Name, ty: &Type, dur: Duration) -> Self {
        Self {
            duration: Some(dur.as_millis()),
            name: name.clone(),
            ty: ty.clone(),
            error: None,
            number,
        }
    }

    pub fn error(number: usize, name: &Name, ty: &Type, error: String) -> Self {
        Self {
            error: Some(error),
            name: name.clone(),
            ty: ty.clone(),
            duration: None,
            number,
        }
    }
}
