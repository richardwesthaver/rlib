//! cfg::config::program
//!
//! Program configuration primitives

use serde::{Deserialize, Serialize};

/// Program configuration type
#[derive(Serialize, Deserialize, Debug, Hash, Default)]
pub struct ProgramConfig {}
