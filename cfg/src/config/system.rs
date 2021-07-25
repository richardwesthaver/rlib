use serde::{Deserialize, Serialize};

use super::user::UserConfig;

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct SystemConfig {}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct SysConfig {
  hostname: String,
  os: String,
  arch: String,
  users: Vec<UserConfig>,
}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct VmConfig {}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct VmmConfig {}
