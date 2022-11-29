use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct RawCollectorOptions {
    pub max_entries: usize,
    pub anonymize: bool,
    pub interval: usize,
    pub backend: String,
    pub enabled: bool,
}

impl Default for RawCollectorOptions {
    fn default() -> Self {
        return Self {
            max_entries: 1000,
            anonymize: false,
            interval: 900,
            backend: String::from("default"),
            enabled: true,
        };
    }
}

// impl RawCollectorOptions {
//     pub fn validate() -> Result<CollectorOptions, CollectorValidationError> {}
// }
