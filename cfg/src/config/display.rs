//! config/display.rs --++-- $DISPLAY config
//!
//! The structs in this module pertain to PHYSICAL Display configurations. They
//! immplement functionality intended for use in OS system and user
//! configurations. For example: xrandr configuration.
use serde::{Deserialize, Serialize};

/// Monitor configuration
pub struct Monitor {
  name: String,
  brightness: u32,
  resolution: (u16, u16),
}

impl Monitor {
  fn new(name: &str, brightness: u32, x: u16, y: u16) -> Self {
    Monitor {
      name: String::from(name),
      brightness,
      resolution: (x, y),
    }
  }
}

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
