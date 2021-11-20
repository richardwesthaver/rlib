//! cmd/shell.rs --- (async) Shell Commands
use crate::Result;
use std::process::Output;
use tokio::{io::Result as CR, process::Command as Cmd};

/// GNU Makefile command
pub async fn make(args: Vec<&str>) -> CR<Output> {
  Cmd::new("make")
    .args(args.into_iter())
    .spawn()
    .expect("make failed.")
    .wait_with_output()
    .await
}

/// GNU Emacs command
pub async fn emacs(args: Vec<&str>) -> CR<Output> {
  Cmd::new("emacs")
    .args(args)
    .spawn()?
    .wait_with_output()
    .await
}

pub async fn emacsclient(args: Vec<&str>) -> CR<Output> {
  Cmd::new("emacsclient")
    .args(args)
    .spawn()?
    .wait_with_output()
    .await
}

/// ffmpeg command
pub async fn ffmpeg(args: Vec<&str>) -> CR<Output> {
  Cmd::new("ffmpeg")
    .args(args)
    .spawn()?
    .wait_with_output()
    .await
}

/// set the desktop background to an image given its absolute path
#[cfg(unix)]
pub async fn fehbg(img_path: &str) -> CR<Output> {
  Cmd::new("feh")
    .args(["--no-fehbg", "--bg-center"])
    .arg(img_path)
    .output()
    .await
}

/// start conky service in background
#[cfg(unix)]
pub async fn conky(cfg: &str) -> CR<Output> {
  Cmd::new("conky")
    .arg("-qbdc")
    .arg(cfg)
    .arg("'&'")
    .output()
    .await
}

/// generate WireGuard keypairs
pub async fn wg_keygen(peers: Vec<&str>) -> Result<()> {
  println!("writing keypairs to {:?}", std::env::current_dir().unwrap());
  for i in peers.iter() {
    let key_file = std::fs::File::create(format!("{}{}", i, ".key")).unwrap();
    let pub_file = std::fs::File::create(format!("{}{}", i, ".pub")).unwrap();
    Cmd::new("wg")
      .arg("genkey")
      .stdout(key_file)
      .output()
      .await
      .expect("could not create private key");
    Cmd::new("wg")
      .arg("pubkey")
      .stdin(std::fs::File::open(format!("{}{}", i, ".key")).unwrap())
      .stdout(pub_file)
      .output()
      .await
      .expect("could not create public key");
  }
  Ok(())
}
