mod error;
mod multi;
mod single;

pub use error::*;
pub use multi::*;
pub use single::*;

pub type ClientResult<T> = Result<T, ClientError>;
