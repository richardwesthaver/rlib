//! cfg::config::user
//!
//! User configuration primitives
use crate::object::{direction::RelativeDirection, location::Point};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{AuthConfig, DisplayConfig, PackageConfig, ProjectConfig};

/// User configuration type
///
/// Used to configure system users for various platforms.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserConfig {
  pub shell: UserShellConfig,
  pub geo: Option<Point>,
  pub displays: Option<Vec<DisplayConfig>>,
  pub packages: Vec<PackageConfig>,
  pub projects: Vec<ProjectConfig>,
  pub auth: Vec<AuthConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserShellConfig {
  pub env: HashMap<String, String>,
  pub cmds: HashMap<String, String>,
  pub shell: ShellType,
}

#[derive(Serialize, Deserialize, Debug, Hash, Default)]
pub enum ShellType {
  #[default]
  Bash,
  Zsh,
  Sh,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TmuxSessionConfig {
  pub windows: Vec<TmuxWindowConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TmuxWindowConfig {
  pub name: String,
  pub panes: Vec<TmuxPaneConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TmuxPaneConfig {
  pub name: String,
  pub position: Option<RelativeDirection>,
  pub init: Option<String>,
}
