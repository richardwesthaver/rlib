mod err;
//pub mod codec;

#[cfg(feature="client")]
pub mod client;

pub use crate::{
  err::Error,
};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
