//! cfg::config::package
//!
//! Package configuration primitives
use serde::{Deserialize, Serialize};

use super::library::LibraryConfig;
use super::program::ProgramConfig;
use super::repo::RepoConfig;
use super::Configure;

/// A single package configuration.
#[derive(Serialize, Deserialize, Hash, Debug, PartialEq)]
pub struct PackageConfig {
  /// Name of this package
  pub name: String,
  pub repo: Option<RepoConfig>,
  pub bin: Option<ProgramConfig>,
  pub lib: Option<LibraryConfig>,
}

impl PackageConfig {
  /// Create a new PackageConfig with a given name
  pub fn new(name: &str) -> Self {
    PackageConfig {
      name: name.to_string(),
      repo: None,
      bin: None,
      lib: None,
    }
  }
}
impl Default for PackageConfig {
  fn default() -> Self {
    PackageConfig {
      name: "".to_string(),
      repo: Some(RepoConfig::default()),
      bin: Some(ProgramConfig::default()),
      lib: Some(LibraryConfig::default()),
    }
  }
}

impl Configure for PackageConfig {}
