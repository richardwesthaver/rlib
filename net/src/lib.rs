//! net client/server library
//!
//! net is used for building client/server programs on any platform.
mod err;
// pub mod codec;
pub mod engine;
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "client")]
pub use client::Client;
#[cfg(feature = "server")]
pub use server::Server;

pub use crate::err::{Error, Result};

#[cfg(test)]
mod tests;
