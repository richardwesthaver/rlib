//! cfg::config::package
//!
//! Package configuration primitives
use serde::{Deserialize, Serialize};
use super::library::LibraryConfig;
use super::program::ProgramConfig;
use super::repo::RepoConfig;
use super::Configure;
use crate::Objective;

use std::collections::HashMap;
/// A single package configuration.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageConfig {
  /// Name of this package
  pub name: String,
  pub repo: Option<RepoConfig>,
  pub bin: Option<ProgramConfig>,
  pub lib: Option<LibraryConfig>,
  pub babel: Option<HashMap<String, Vec<String>>>,
  pub meta: Option<String>, // will be MetaConfig
}

impl PackageConfig {
  /// Create a new PackageConfig with a given name
  pub fn new(name: &str) -> Self {
    PackageConfig {
      name: name.to_string(),
      repo: None,
      bin: None,
      lib: None,
      babel: None,
      meta: None,
    }
  }
}
impl Default for PackageConfig {
  fn default() -> Self {
    PackageConfig {
      name: "default".to_string(),
      repo: Some(RepoConfig::default()),
      bin: Some(ProgramConfig::default()),
      lib: Some(LibraryConfig::default()),
      babel: None,
      meta: None,
    }
  }
}

impl Configure for PackageConfig {}
impl Objective for PackageConfig {}
