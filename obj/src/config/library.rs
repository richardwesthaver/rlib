//! cfg::config::library
//!
//! Library configuration primitives

use super::Configure;
use crate::Objective;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Software library configuration
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LibraryConfig {
  name: String,
  scripts: HashMap<String, String>,
}

impl Configure for LibraryConfig {}
impl Objective for LibraryConfig {}
