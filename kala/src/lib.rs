//! kala desktop command library
//!
//! This module provides collections of system functions available via feature
//! flags.
//!
//! notes regarding platform support:
//! - the default target (all features supported) is GNU Linux
//! - features that do not include external dependencies run on all platforms
//!   (with stdlib support)
//! - some features *only* support unix, others *only* support GNU Linux
//!   specifically
pub mod cmd;
#[cfg(feature = "dmc")]
pub mod dmc;
mod err;
#[cfg(feature = "python")]
pub mod python;
pub use err::{Error, Result};
