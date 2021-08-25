//! cfg::config::program
//!
//! Program configuration primitives

use serde::{Deserialize, Serialize};
use super::Configure;
/// Program configuration type
#[derive(Serialize, Deserialize, Debug, Hash, Default, PartialEq)]
pub struct ProgramConfig {}

impl Configure for ProgramConfig {}
