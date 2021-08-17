//! cfg::config::package
//!
//! Package configuration primitives
use serde::{Deserialize, Serialize};

use super::library::LibraryConfig;
use super::program::ProgramConfig;
use super::repo::RepoConfig;

/// A single package configuration.
#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct PackageConfig {
  /// Name of this package
  name: String,
  repo: Option<RepoConfig>,
  program: Option<ProgramConfig>,
  library: Option<LibraryConfig>,
}

impl PackageConfig {
  /// Create a new PackageConfig with a given name
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
