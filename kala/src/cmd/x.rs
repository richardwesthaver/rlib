//! cmd/x.rs --- X11 commands
//!
//! This module will only compile on GNU Linux.
use tokio::{io::Result as CR, process::Command as Cmd};
use std::process::Output;
use logger::log::{debug, info, trace};
use obj::DisplayConfig;
use std::env;
// this import depends on x11 crate. the x11-sys build script fails on
// non-gnu linux targets (including musl)
use xrandr::XHandle;

/// start X11 server after ensuring DISPLAY is unset, then put the
/// thread to sleep for a short duration while waiting for init.
///
/// TODO: [2021-08-13 22:39] - spawn this in background thread, +remove cmd_lib+
pub async fn startx() {
  // check to see if DISPLAY is set, else start the X server
  if let Ok(val) = env::var("DISPLAY") {
    debug!("display {} is already set. skipping call to startx.", val);
  } else {
    info!("DISPLAY unset, starting X server.");
    Cmd::new("sh")
      .args(["-c","'startx'"])
      .output()
      .await.expect("X didn't give it to ya");
  }
}

/// List available monitors via X11 with verbose output
///
/// Use this for debugging only.
pub fn xrandr_list() {
  let monitors = XHandle::open().unwrap().monitors().unwrap();
  println!("{:#?}", monitors);
}

/// Configure a Display with xrandr
pub async fn xrandr(cfg: DisplayConfig) -> CR<Output> {
  trace!("{:#?}", cfg);
  let mode = format!("{}x{}", cfg.resolution.0, cfg.resolution.1);
  let mut args = vec![
    "--output",
    &cfg.output,
    "--mode",
    &mode,
    "--pos",
    &cfg.pos,
    "--rotate",
    &cfg.rotate,
  ];

  if cfg.primary == true {
    args.push("--primary");
  }
  Cmd::new("xrandr")
    .args(args)
    .output()
    .await
}
