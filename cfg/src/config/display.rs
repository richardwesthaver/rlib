//! cfg::config::display
//!
//! The structs in this module pertain to PHYSICAL Display
//! configurations. They immplement functionality intended for use in
//! OS system and user configurations. For example: xrandr
//! configuration.
use serde::{Deserialize, Serialize};
use super::Configure;
/// Monitor configuration type
pub struct MonitorConfig {
  pub name: String,
  pub brightness: u32,
  pub resolution: (u16, u16),
}

impl MonitorConfig {
  /// Create a new Monitor configuration
  pub fn new(name: &str, brightness: u32, x: u16, y: u16) -> Self {
    MonitorConfig {
      name: String::from(name),
      brightness,
      resolution: (x, y),
    }
  }
}

impl Configure for MonitorConfig {}

/// Display configuration
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct DisplayConfig {
  pub output: String,
  pub primary: bool,
  pub mode: String,
  pub pos: String,
  pub rotate: String,
}

impl Default for DisplayConfig {
  fn default() -> Self {
    DisplayConfig {
      output: "".to_string(),
      primary: true,
      mode: "1920x1080".to_string(),
      pos: "0x0".to_string(),
      rotate: "normal".to_string(),
    }
  }
}

impl Configure for DisplayConfig {}
