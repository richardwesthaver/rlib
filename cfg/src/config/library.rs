//! cfg::config::library
//!
//! Library configuration primitives

use serde::{Deserialize, Serialize};

/// Software library configuration
#[derive(Serialize, Deserialize, Debug, Hash, Default)]
pub struct LibraryConfig {}
