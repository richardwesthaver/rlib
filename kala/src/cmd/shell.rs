use crate::Result;
use cmd_lib::{run_cmd, CmdResult};
use std::collections::HashMap;
use std::process::{Command, Output, Stdio};

pub fn make(target: &str) {
  let make = Command::new("make")
    .arg(target)
    .stdin(Stdio::null())
    .stdout(Stdio::inherit())
    .status()
    .expect("make failed.");
  println!("make finished with {}", make);
}

pub fn emacs(args: Vec<&str>, envs: HashMap<&str, &str>) -> Result<Output> {
  Ok(
    Command::new("emacs")
      .env_clear()
      .envs(envs)
      .args(args)
      .spawn()?
      .wait_with_output()?,
  )
}

pub fn ffmpeg(args: Vec<&str>, envs: HashMap<&str, &str>) -> Result<Output> {
  Ok(
    Command::new("ffmpeg")
      .env_clear()
      .envs(envs)
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
#[cfg(unix)]
pub fn fehbg(img_path: &str) -> CmdResult {
  Ok(run_cmd!(feh --no-fehbg --bg-center "$img_path")?)
}

/// start conky service in background
#[cfg(unix)]
pub fn conky(cfg: &str) -> CmdResult {
  Ok(run_cmd!(conky  -qbdc "$cfg" '&')?)
}
