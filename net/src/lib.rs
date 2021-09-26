//! net client/server library
//!
//! net is used for building client/server programs on any platform.
mod err;
pub use err::{Error, Result};

// pub mod codec;
#[cfg(feature = "client")]
mod client;
pub mod coding;
pub mod connection;
pub mod engine;
#[cfg(feature = "client")]
pub use client::Client;
#[cfg(feature = "client")]
pub use reqwest;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::FileServer;
#[cfg(feature = "server")]
pub use server::Server;

#[cfg(test)]
mod tests;
