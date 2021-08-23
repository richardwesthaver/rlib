//! Kalash - simple commands
//!
//! NOTE [2021-07-26 Mon]: clap code is officially being migrated to a separate
//! crate.

pub mod cmd;
mod err;
pub mod repl;
pub use crate::err::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
