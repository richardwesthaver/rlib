//! kalash::cmd
use core::fmt;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;

use cfg::repo::HgWebConfig;
use cfg::DisplayConfig;
use cmd_lib::{run_cmd, spawn_with_output, use_builtin_cmd, CmdResult, FunResult};
use logger::log::{error, info, trace};
use xrandr::XHandle;

use crate::Result;
pub fn hgweb(cfg: HgWebConfig) -> CmdResult {
  use_builtin_cmd!(echo, info);

  // Continuously process child process' outputs
  spawn_with_output!(hgweb)?.wait_with_pipe(&mut |pipe| {
    let cfg = option_env!("SHED_CFG").expect("$SHED_CFG undefined");
    BufReader::new(pipe)
      .lines()
      .filter_map(|line| line.ok())
      .take(10)
      .for_each(|line| info!("{}", line));
  });
  Ok(())
}

pub fn startx() -> CmdResult {
  // check to see if DISPLAY is set, else start the X server
  if let Ok(val) = env::var("DISPLAY") {
    error!("DISPLAY is already set! skipping call to startx.");
  } else {
    info!("DISPLAY unset, starting X server.");
    run_cmd!( sh -c "startx" )?;
  }
  Ok(())
}

pub fn xrandr_list() {
  let monitors = XHandle::open().unwrap().monitors().unwrap();
  info!("{:#?}", monitors);
}

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
      .spawn()
      .unwrap()
      .wait_with_output()?,
  )
}

/// start the mpd daemon in background
pub fn mpd() -> CmdResult {
  Ok(run_cmd!(mpd)?)
}

/// set the desktop background to an image given its absolute path
pub fn fehbg(img_path: &str) -> CmdResult {
  Ok(run_cmd!(feh --no-fehbg --bg-center $img_path)?)
}
