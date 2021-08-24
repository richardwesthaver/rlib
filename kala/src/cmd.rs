//! kalash::cmd
//!
//! collection of wrapper commands for various functions.
//!
//! TODO [2021-08-23 Mon 21:56] - check all modules for
//! target-specific dependencies and add flags.
pub mod hg;
pub mod midi;
pub mod pass;
pub mod shell;
pub mod tmux;
pub mod usb;
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub mod x;
