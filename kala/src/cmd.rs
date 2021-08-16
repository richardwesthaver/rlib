//! kalash::cmd
use std::env;
use std::io::BufRead;
use std::io::BufReader;

use cfg::repo::HgWebConfig;
use cfg::DisplayConfig;
use cmd_lib::log::debug;
use cmd_lib::{run_cmd, spawn_with_output, use_builtin_cmd, CmdResult};
use logger::log::{info, trace};

#[cfg(all(target_os="linux", target_env="gnu"))]
use xrandr::XHandle;

use crate::Result;

pub fn hgweb(cfg: HgWebConfig) -> CmdResult {
  println!("found hgweb_config: {:?}", cfg);
  use_builtin_cmd!(echo, info);
  // Continuously process child outputs
  spawn_with_output!(hgweb)?.wait_with_pipe(&mut |pipe| {
    let cfg = option_env!("SHED_CFG").expect("$SHED_CFG undefined");
    info!("found config {}", cfg);
    BufReader::new(pipe)
      .lines()
      .filter_map(|line| line.ok())
      .take(10)
      .for_each(|line| info!("{}", line));
  });
  Ok(())
}

/// start X11 server after ensuring DISPLAY is unset, then put the
/// thread to sleep for a short duration while waiting for init.
///
/// TODO: [2021-08-13 22:39] - spawn this in background thread, remove cmd_lib
#[cfg(target_os = "linux")]
pub fn startx() -> CmdResult {
  // check to see if DISPLAY is set, else start the X server
  if let Ok(val) = env::var("DISPLAY") {
    debug!("display {} is already set. skipping call to startx.", val);
  } else {
    info!("DISPLAY unset, starting X server.");
    run_cmd!( sh -c "startx" )?;
    std::thread::sleep(std::time::Duration::from_secs(4));
  }
  Ok(())
}


/// List available monitors via X11
#[cfg(all(target_os = "linux", target_env="gnu"))]
pub fn xrandr_list() {
  let monitors = XHandle::open().unwrap().monitors().unwrap();
  info!("{:#?}", monitors);
}

/// Configure a Display with xrandr
#[cfg(all(target_os = "linux", target_env="gnu"))]
pub fn xrandr(cfg: DisplayConfig) -> Result<std::process::Output> {
  trace!("{:#?}", cfg);
  let mut args = vec![
    "--output",
    &cfg.output,
    "--mode",
    &cfg.mode,
    "--pos",
    &cfg.pos,
    "--rotate",
    &cfg.rotate,
  ];

  if cfg.primary == true {
    args.push("--primary");
  }

  Ok(
    std::process::Command::new("xrandr")
      .args(args)
      .spawn()?
      .wait_with_output()?,
  )
}

/// start the mpd daemon in background
#[cfg(unix)]
pub fn mpd() -> CmdResult {
  Ok(run_cmd!(mpd)?)
}

/// set the desktop background to an image given its absolute path
pub fn fehbg(img_path: &str) -> CmdResult {
  Ok(run_cmd!(feh --no-fehbg --bg-center "$img_path")?)
}

/// start conky service in background
#[cfg(unix)]
pub fn conky(cfg: &str) -> CmdResult {
  Ok(run_cmd!(conky  -qbdc "$cfg" '&')?)
}
