//! kalash::cmd
//!
//! collection of wrapper commands for various functions.
pub mod midi;
pub mod pass;
pub mod shell;
pub mod tmux;
pub mod usb;
#[cfg(all(target_os="linux", target_env="gnu"))]
pub mod x;

use cfg::repo::HgwebConfig;
use cmd_lib::{run_cmd, spawn_with_output, use_builtin_cmd, CmdResult};
use logger::log::info;
use std::io::BufRead;
use std::io::BufReader;

pub fn hgweb(cfg: HgwebConfig) -> CmdResult {
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
