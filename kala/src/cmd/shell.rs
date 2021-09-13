//! Shell command module
use crate::Result;
use cmd_lib::{run_cmd, CmdResult};
use std::collections::HashMap;
use std::process::{Command, Output, Stdio};

/// GNU Makefile command
pub fn make(target: &str) {
  let make = Command::new("make")
    .arg(target)
    .stdin(Stdio::null())
    .stdout(Stdio::inherit())
    .status()
    .expect("make failed.");
  println!("make finished with {}", make);
}

/// GNU Emacs command
pub fn emacs(args: Vec<&str>) -> Result<Output> {
  Ok(
    Command::new("emacs")
      .args(args)
      .spawn()?
      .wait_with_output()?
  )
}

/// ffmpeg command
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

pub fn wg_keygen(peers: Vec<&str>) -> Result<()> {
  println!("writing keypairs to {:?}", std::env::current_dir().unwrap());
  for i in peers.iter() {
    let key_file = std::fs::File::create(format!("{}{}", i, ".key")).unwrap();
    let pub_file = std::fs::File::create(format!("{}{}", i, ".pub")).unwrap();
    Command::new("wg")
      .arg("genkey")
      .stdout(key_file)
      .output()
      .expect("could not create private key");

    Command::new("wg")
      .arg("pubkey")
      .stdin(std::fs::File::open(format!("{}{}", i, ".key")).unwrap())
      .stdout(pub_file)
      .output()
      .expect("could not create public key");

    println!("{} -- done", i);
  }

  Ok(())
}
