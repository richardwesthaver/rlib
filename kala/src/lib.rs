//! Kalash - simple commands
//!
//! NOTE [2021-07-26 Mon]: clap code is officially being migrated to a separate
//! crate.

pub mod cmd;
pub mod ctl;
mod err;
mod repl;
pub mod tmux;
use crate::err::Error;
pub type Result<T> = std::result::Result<T, Error>;
pub use rpassword::{prompt_password, read_password, read_password_from_bufread};

pub use crate::repl::repl;

#[cfg(test)]
mod tests;
