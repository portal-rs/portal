use serde::Serialize;

use crate::bench::BenchResult;

#[derive(Debug, Serialize)]
pub struct BenchSummary {
    pub results: Vec<BenchResult>,
    pub server: String,
    pub delay: usize,
    pub runs: usize,
}
