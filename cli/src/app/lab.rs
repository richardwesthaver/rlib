use super::{
  cmd::{load_cfg_cmd, make_cmd, pack_cmd, write_cfg_cmd},
  App, AppSettings, Arg, Cmd, SubCommand,
};
use crate::ctl::{load_config, pack, shell::make, write_config};
use crate::Result;
/// CLI container for shedctl
pub struct LabCli {
  pub args: Vec<Arg>,
  pub cmds: Vec<Cmd>,
}

impl LabCli {
  pub fn new() -> Self {
    let args = LabArgs::new();
    let cmds = vec![pack_cmd(), write_cfg_cmd(), load_cfg_cmd(), make_cmd()];
    LabCli { args: args.0, cmds }
  }

  pub fn build(&self) -> Result<Cmd> {
    let cmds = &self.cmds;
    let args = &self.args;
    let app = App::new("labrat")
      .author("ellis")
      .about("CREATE AND DESTROY")
      .subcommands(cmds.clone())
      .args(args);
    Ok(app)
  }

  pub fn run(&self) -> Result<()> {
    let app = self.build()?;
    let matches = app.get_matches();
    Ok(())
  }
}

impl Default for LabCli {
  fn default() -> Self {
    Self::new()
  }
}

pub struct LabArgs(Vec<Arg>);

impl LabArgs {
  pub fn new() -> Self {
    let mut args = vec![LabArgs::config_arg()];
    LabArgs(args)
  }

  pub fn config_arg() -> Arg {
    Arg::with_name("config")
      .short("c")
      .long("config")
      .takes_value(true)
      .value_name("RON_FILE")
      .help("Specifies the config.ron file to use.")
  }
}
