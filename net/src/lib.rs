//! net client/server library
//!
//! net is used for building client/server programs on any platform.
mod err;
//mod codec;
#[cfg(feature = "client")]
pub mod client;

pub use crate::err::{Error, Result};

#[cfg(test)]
mod tests;
