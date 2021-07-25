//! config/package --++-- Package configuration
//!
//! Naive package configuration format for file-based package management.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::library::LibraryConfig;
use super::program::ProgramConfig;
use super::repo::RepoConfig;

/// A single package configuration.
#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct PackageConfig {
  name: String,
  repo: Option<RepoConfig>,
  program: Option<ProgramConfig>,
  library: Option<LibraryConfig>,
}

impl PackageConfig {
  pub fn new(name: &str) -> Self {
    PackageConfig {
      name: name.to_string(),
      repo: None,
      program: None,
      library: None,
    }
  }
}
impl Default for PackageConfig {
  fn default() -> Self {
    PackageConfig {
      name: "".to_string(),
      repo: Some(RepoConfig::default()),
      program: Some(ProgramConfig::default()),
      library: Some(LibraryConfig::default()),
    }
  }
}
