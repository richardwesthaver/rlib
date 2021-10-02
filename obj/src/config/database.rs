//! cfg::config::database
//!
//! Database configuration primitives
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct DatabaseConfig {
  engine: String,
}
