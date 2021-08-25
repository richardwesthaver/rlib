//! cfg::config::library
//!
//! Library configuration primitives

use serde::{Deserialize, Serialize};
use super::Configure;
/// Software library configuration
#[derive(Serialize, Deserialize, Debug, Hash, Default, PartialEq)]
pub struct LibraryConfig {}

impl Configure for LibraryConfig {}
