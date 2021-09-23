//! cfg::config::repo
//!
//! Repo configuration primitives
use super::Configure;
use crate::Objective;
use std::{
  fmt,
  path::PathBuf,
};

use serde::{Deserialize, Serialize};
#[cfg(feature = "hg")]
pub mod hg;
#[cfg(feature = "git")]
mod git;
/// Generic repo configuration type
///
/// Wraps Mercurial and/or Git repos
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq)]
pub struct RepoConfig {
  pub vcs: String,
  pub origin: String,
  pub path: PathBuf,
}

impl RepoConfig {
  /// Create a new RepoConfig
  pub fn new() -> Self {
    RepoConfig::default()
  }
}

impl Default for RepoConfig {
  fn default() -> Self {
    RepoConfig {
      vcs: "hg".to_string(),
      origin: "".to_string(),
      path: PathBuf::from("."),
    }
  }
}

impl Configure for RepoConfig {}
impl Objective for RepoConfig {}

/// The type of Repo (Mercurial or Git)
pub enum RepoType {
  MercurialRepository,
  GitRepository,
}

impl fmt::Display for RepoType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RepoType::MercurialRepository => write!(f, "hg"),
      RepoType::GitRepository => write!(f, "git"),
    }
  }
}

/// Subrepo type
///
/// Note that Mercurial subrepos are a 'feature of last resort'
/// according to official docs. They are needed in very niche
/// scenarios and shouldn't be used most of the time.
pub struct SubRepo {
  pub vcs: String,
  pub origin: String,
  pub path: String,
}

