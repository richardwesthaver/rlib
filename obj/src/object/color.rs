use super::{Deserialize, Objective, Serialize};
/// Color types, repr in RGBA8,16,32 layout later on
#[derive(Serialize, Deserialize, Debug, Hash, Default)]
pub struct Color {
  red: u8,
  green: u8,
  blue: u8,
  alpha: u8,
}

impl Objective for Color {}
