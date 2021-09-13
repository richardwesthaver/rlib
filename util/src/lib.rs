//! util library
#[cfg(feature = "anyhow")]
pub use anyhow::Result;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "fmt")]
pub mod fmt;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "bs")]
pub mod bs;
pub mod path;
#[cfg(test)]
mod tests;
