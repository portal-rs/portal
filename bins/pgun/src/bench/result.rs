use std::time::Duration;

use portal::types::{
    dns::Name,
    rr::{Record, Type},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BenchResult {
    pub answers: Option<Vec<Record>>,
    pub duration: Option<u128>,
    pub error: Option<String>,
    pub number: usize,
    pub name: Name,

    #[serde(rename = "type")]
    pub ty: Type,
}

impl BenchResult {
    pub fn success(
        number: usize,
        name: &Name,
        ty: &Type,
        answers: &Vec<Record>,
        dur: Duration,
    ) -> Self {
        Self {
            duration: Some(dur.as_millis()),
            answers: Some(answers.clone()),
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
            answers: None,
            number,
        }
    }
}
