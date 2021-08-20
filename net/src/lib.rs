mod err;
//pub mod codec;
pub use crate::{
  err::Error,
};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
