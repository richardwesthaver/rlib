//! cfg::config::library
//!
//! Library configuration primitives

use super::Configure;
use crate::Objective;

use serde::{Deserialize, Serialize};
/// Software library configuration
#[derive(Serialize, Deserialize, Debug, Hash, Default, PartialEq)]
pub struct LibraryConfig {}

impl Configure for LibraryConfig {}
impl Objective for LibraryConfig {}
