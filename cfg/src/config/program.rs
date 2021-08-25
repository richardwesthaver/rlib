//! cfg::config::program
//!
//! Program configuration primitives

use super::Configure;
use serde::{Deserialize, Serialize};
/// Program configuration type
#[derive(Serialize, Deserialize, Debug, Hash, Default, PartialEq)]
pub struct ProgramConfig {}

impl Configure for ProgramConfig {}
