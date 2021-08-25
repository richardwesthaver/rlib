//! cfg::config::database
//!
//! Database configuration primitives

use serde::{Deserialize, Serialize};
use super::Configure;
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct DatabaseConfig {}

impl Configure for DatabaseConfig {}
