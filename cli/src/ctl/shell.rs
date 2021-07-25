use std::collections::HashMap;
use std::process::{Command, Output, Stdio};

use cmd_lib::run_cmd;

use crate::Result;

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
