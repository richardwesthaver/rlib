//! cfg::config::user
//!
//! User configuration primitives
use crate::object::direction::RelativeDirection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// User configuration type
///
/// Used to configure system users for various platforms.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserConfig {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserShellConfig {
  env: HashMap<String, String>,
  cmds: HashMap<String, String>,
  shell: ShellType,
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
  windows: Vec<TmuxWindowConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TmuxWindowConfig {
  name: String,
  panes: Vec<TmuxPaneConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TmuxPaneConfig {
  name: String,
  position: Option<RelativeDirection>,
  init: Option<String>,
}
