//! # Temperature object
//!
//! Generic temperatures (F/C)
//! TODO: impl From<Celsius/Fahrenheit> using conversions
use crate::Objective;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash)]
pub enum Temperature {
  Fahrenheit(u32),
  Celsius(u32),
}

#[derive(Serialize, Deserialize, Debug, Hash, Default)]
pub struct Celsius(u32);

#[derive(Serialize, Deserialize, Debug, Hash, Default)]
pub struct Fahrenheit(u32);

impl Objective for Temperature {}
