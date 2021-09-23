//! cfg::config::database
//!
//! Database configuration primitives

use super::Configure;
use crate::Objective;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct DatabaseConfig {
  engine: String,
}

impl Configure for DatabaseConfig {}
impl Objective for DatabaseConfig {}
