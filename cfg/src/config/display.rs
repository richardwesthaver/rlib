//! config/display.rs --++-- $DISPLAY config
//!
//! The structs in this module pertain to PHYSICAL Display configurations. They
//! immplement functionality intended for use in OS system and user
//! configurations. For example: xrandr configuration.

struct Monitor {
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
