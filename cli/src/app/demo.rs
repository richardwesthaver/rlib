use std::path::Path;

use sys::logger::log;

use super::{
  cmd::{load_cfg_cmd, make_cmd, pack_cmd, usb_cmd, weather_cmd, write_cfg_cmd},
  App, AppSettings, Arg, Cmd, SubCommand,
};
use crate::ctl::{load_config, pack, shell::make, usb_devices, weather_report, write_config};
use crate::Result;
/// CLI container for shedctl
pub struct DemoCli {
  pub args: Vec<Arg>,
  pub cmds: Vec<Cmd>,
}

impl DemoCli {
  pub fn new() -> Self {
    let args = DemoArgs::new();
    let cmds = vec![pack_cmd(), make_cmd(), usb_cmd(), weather_cmd()];
    DemoCli { args: args.0, cmds }
  }

  pub fn build(&self) -> Result<Cmd> {
    let cmds = &self.cmds;
    let args = &self.args;
    let app = App::new("demo")
      .author("ellis")
      .about("ANTI_CORP DEMO V0.1.0")
      .global_settings(&[
        AppSettings::ColoredHelp,
        AppSettings::VersionlessSubcommands,
        AppSettings::UnifiedHelpMessage,
        AppSettings::AllowMissingPositional,
      ])
      .subcommands(cmds.clone())
      .args(args);
    Ok(app)
  }

  pub async fn run(&self) -> Result<()> {
    let app = self.build()?;
    let matches = app.get_matches();

    // pack command
    if let Some(matches) = matches.subcommand_matches("pack") {
      if matches.is_present("input") {
        log::info!(
          "input: {}",
          matches.value_of("input").expect("illegal input!")
        );
        pack(
          Path::new(matches.value_of("input").unwrap()),
          Path::new(matches.value_of("output").unwrap()),
        );
      } else {
        let cd = std::env::current_dir().unwrap();
        println!("using default input");
        pack(cd.as_path(), Path::new(matches.value_of("output").unwrap()));
      }
    };

    // make command
    if let Some(matches) = matches.subcommand_matches("make") {
      if matches.is_present("target") {
        make(matches.value_of("target").expect("bad target :("));
      } else {
        make(" ");
      }
    };

    // usb devices
    if let Some(matches) = matches.subcommand_matches("usb") {
      usb_devices();
    }

    // usb devices
    if let Some(matches) = matches.subcommand_matches("weather") {
      weather_report().await;
    }

    // all jobs done
    println!("--++--");
    Ok(())
  }
}

impl Default for DemoCli {
  fn default() -> Self {
    Self::new()
  }
}

pub struct DemoArgs(Vec<Arg>);

impl DemoArgs {
  pub fn new() -> Self {
    Self::default()
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

impl Default for DemoArgs {
  fn default() -> Self {
    let mut args = vec![DemoArgs::config_arg()];
    DemoArgs(args)
  }
}
