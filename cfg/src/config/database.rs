//! cfg::config::database
//!
//! Database configuration primitives

use super::Configure;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct DatabaseConfig {}

impl Configure for DatabaseConfig {}
