#[macro_export]
macro_rules! cast {
    ($Input:expr, $As:path) => {
        match $Input {
            $As(inner) => Some(inner),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! cast_or {
    ($Input:expr, $As:path, $Or:expr) => {
        match $Input {
            $As(inner) => inner,
            _ => $Or,
        }
    };
}
