//! cfg::config::program
//!
//! Program configuration primitives
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::RepoConfig;

/// Program configuration type
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct ProgramConfig {
  pub name: String,
  pub path: String,
  pub src: RepoConfig,
  pub build: Option<String>,
  pub scripts: HashMap<String, String>,
}
