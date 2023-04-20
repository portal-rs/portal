#[derive(Debug)]
pub enum IpVersion {
    Both,
    V4,
    V6,
}

impl Default for IpVersion {
    fn default() -> Self {
        Self::Both
    }
}

impl From<(bool, bool)> for IpVersion {
    fn from(value: (bool, bool)) -> Self {
        if !value.0 && !value.1 {
            return Self::Both;
        } else if !value.0 && value.1 {
            return Self::V6;
        } else {
            Self::V4
        }
    }
}
