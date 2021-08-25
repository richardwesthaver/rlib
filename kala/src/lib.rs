//! kalash desktop command library
//!
//! This module provides collections of cli functions available via feature flags.
//!
//! Some notes regarding platform support:
//! - the default target (all features supported) is GNU Linux
//! - features that do not include external dependencies (only on `rlib` modules)
//!   run on all platforms (with stdlib support)
//! - some features *only* support unix, others *only* support GNU Linux specifically
//! - Windows isn't officially supported
pub mod cmd;
mod err;
pub use crate::err::Error;
pub type Result<T> = std::result::Result<T, Error>;
