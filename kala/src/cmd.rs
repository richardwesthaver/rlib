//! cmd.rs --- kalash commands
//!
//! collection of wrapper wrapper functions for system features

#[cfg(feature = "hg")]
pub mod hg;
pub mod shell;

#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "midi")]
pub mod midi;
pub mod repl;
#[cfg(feature = "sys")]
pub mod sys;
#[cfg(feature = "tmux")]
pub mod tmux;
#[cfg(all(target_os = "linux", target_env = "gnu", feature = "x11"))]
pub mod x;
