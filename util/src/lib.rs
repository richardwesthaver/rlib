//! util library
#[cfg(feature = "anyhow")]
pub use anyhow::Result;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "fmt")]
pub mod fmt;
#[cfg(feature = "time")]
pub mod time;

pub mod path;

#[cfg(test)]
mod tests;
