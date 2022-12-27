use std::fmt::Display;

pub enum Status {
    Hit,
    Miss,
    Expired,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Hit => write!(f, "HIT"),
            Status::Miss => write!(f, "MISS"),
            Status::Expired => write!(f, "EXPIRE"),
        }
    }
}
