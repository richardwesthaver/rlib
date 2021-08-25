//! kalash command library
pub mod cmd;
mod err;
pub use crate::err::Error;
pub type Result<T> = std::result::Result<T, Error>;
