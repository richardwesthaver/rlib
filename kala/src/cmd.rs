//! kalash::cmd
//!
//! collection of wrapper commands for various functions.
//!
//! TODO [2021-08-23 Mon 21:56] - check all modules for
//! target-specific dependencies and add flags.
pub mod shell;
pub mod hg;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "tmux")]
pub mod tmux;
#[cfg(feature = "sys")]
pub mod sys;
#[cfg(feature = "midi")]
pub mod midi;
#[cfg(all(target_os = "linux", target_env = "gnu", feature = "x11"))]
pub mod x;
