//! cfg::config::library
//!
//! Library configuration primitives

use super::Configure;
use serde::{Deserialize, Serialize};
/// Software library configuration
#[derive(Serialize, Deserialize, Debug, Hash, Default, PartialEq)]
pub struct LibraryConfig {}

impl Configure for LibraryConfig {}
