//! net client/server library
//!
//! net is used for building client/server programs on any platform.
mod err;
//mod codec;
mod client;
mod server;
pub use crate::err::{Error, Result};
#[cfg(feature = "client")]
pub use client::Client;
#[cfg(feature = "server")]
pub use server::Server;

#[cfg(test)]
mod tests;
