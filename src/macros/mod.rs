macro_rules! cast {
    ($Input:expr, $As:path) => {
        match $Input {
            $As(inner) => Some(inner),
            _ => None,
        }
    };
}

macro_rules! cast_or {
    ($Input:expr, $As:path, $Or:expr) => {
        match $Input {
            $As(inner) => inner,
            _ => $Or,
        }
    };
}

pub(crate) use cast;
pub(crate) use cast_or;
