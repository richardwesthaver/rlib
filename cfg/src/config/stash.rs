use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct StashConfig {
  path: Option<PathBuf>,
}
