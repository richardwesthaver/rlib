use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct LabConfig {
  path: Option<PathBuf>,
}
