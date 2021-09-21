//! cfg::config::user
//!
//! User configuration primitives
use serde::{Deserialize, Serialize};

/// User configuration type
///
/// Used to configure system users for various platforms.
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct UserConfig {}
