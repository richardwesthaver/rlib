//! cfg::config::program
//!
//! Program configuration primitives
use serde::{Deserialize, Serialize};
/// Program configuration type
///
/// This config is used for modifying how a `Program` is built.
#[derive(Serialize, Deserialize, Debug, Hash, Default, PartialEq)]
pub struct ProgramConfig {
  name: String,
  install_path: Option<String>,
  compiler: Option<String>,
  flags: Option<Vec<String>>,
  build: Option<String>,
  install: Option<String>,
}
