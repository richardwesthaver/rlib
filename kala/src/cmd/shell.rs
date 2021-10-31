//! Shell command module
use crate::Result;
use cmd_lib::{run_cmd, CmdResult};
use ctx::tokio::{io::Result as CR, process::Command};
use std::{collections::HashMap, process::Output};
/// GNU Makefile command
pub async fn make(args: Vec<&str>) -> CR<Output> {
  Command::new("make")
    .args(args.into_iter())
    .spawn()
    .expect("make failed.")
    .wait_with_output()
    .await
}

/// GNU Emacs command
pub async fn emacs(args: Vec<&str>) -> CR<Output> {
  Command::new("emacs")
    .args(args)
    .spawn()?
    .wait_with_output()
    .await
}

pub async fn emacsclient(args: Vec<&str>) -> CR<Output> {
  Command::new("emacsclient")
    .args(args)
    .spawn()?
    .wait_with_output()
    .await
}

/// ffmpeg command
pub async fn ffmpeg(args: Vec<&str>, envs: HashMap<&str, &str>) -> CR<Output> {
  Command::new("ffmpeg")
    .env_clear()
    .envs(envs)
    .args(args)
    .spawn()?
    .wait_with_output()
    .await
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

pub async fn wg_keygen(peers: Vec<&str>) -> Result<()> {
  println!("writing keypairs to {:?}", std::env::current_dir().unwrap());
  for i in peers.iter() {
    let key_file = std::fs::File::create(format!("{}{}", i, ".key")).unwrap();
    let pub_file = std::fs::File::create(format!("{}{}", i, ".pub")).unwrap();
    Command::new("wg")
      .arg("genkey")
      .stdout(key_file)
      .output()
      .await
      .expect("could not create private key");

    Command::new("wg")
      .arg("pubkey")
      .stdin(std::fs::File::open(format!("{}{}", i, ".key")).unwrap())
      .stdout(pub_file)
      .output()
      .await
      .expect("could not create public key");
  }

  Ok(())
}
