//! kalash::cmd
//!
//! collection of wrapper wrapper functions for system features
//!
//! TODO [2021-08-23 Mon 21:56] - check all modules for
//! target-specific dependencies and add flags.

#[cfg(feature = "hg")]
pub mod hg;
pub mod shell;

#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "midi")]
pub mod midi;
#[cfg(feature = "sys")]
pub mod sys;
#[cfg(feature = "tmux")]
pub mod tmux;
#[cfg(all(target_os = "linux", target_env = "gnu", feature = "x11"))]
pub mod x;
pub mod repl;
