//! cmd.rs --- kala commands
//!
//! collection of .. commands

#[cfg(feature = "hg")]
pub mod hg;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "midi")]
pub mod midi;
pub mod repl;
pub mod shell;
#[cfg(feature = "sys")]
pub mod sys;
#[cfg(feature = "tmux")]
pub mod tmux;
#[cfg(all(target_os = "linux", target_env = "gnu", feature = "x11"))]
pub mod x;
