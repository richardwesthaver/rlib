//! net client/server library
//!
//! net is used for building client/server programs on any platform.
mod err;
// pub mod codec;
pub mod connection;
pub mod engine;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::Client;
#[cfg(feature = "client")]
pub use reqwest;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::Server;
#[cfg(feature = "server")]
pub use server::FileServer;
pub use crate::err::{Error, Result};

#[cfg(test)]
mod tests;
