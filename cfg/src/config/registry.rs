//! cfg::config::registry
//!
//! Registry configuration primitives

use serde::{Deserialize, Serialize};

/// Registry configuration type
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct RegistryConfig {}
