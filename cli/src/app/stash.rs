use cfg::{NetworkConfig, PackageConfig, ShedConfig};

use super::{App, AppSettings, Arg, Cmd, SubCommand};
use crate::ctl::{load_config, pack, shell::make, write_config};

/// CLI container for the stash
pub struct StashCli {
  args: Vec<Arg>,
  cmds: Vec<Cmd>,
}

impl StashCli {
  pub fn new() -> Self {
    let mut args = Vec::new();
    let mut cmds = Vec::new();
    StashCli { args, cmds }
  }
}
