//! Mercurial command module
use crate::Result;
use ctx::tokio::process::Command;
use std::process::Output;
use logger::log::debug;
use obj::config::MercurialConfig;

/// Given a `HgwebConfig` struct, start the hgweb server and log to
/// stdout.
/// TODO: import old shed_multi_server
pub async fn hgweb(cfg: &MercurialConfig) -> Result<()> {
  debug!("found hgrc: {:?}", cfg);
  let output = Command::new("hg").arg("serve").output();
  output.await?;

  Ok(())
}

pub async fn hg(args: Vec<&str>) -> Output {
  Command::new("hg")
    .args(args.into_iter())
    .spawn()
    .expect("mercurial error")
    .wait_with_output()
    .await
    .expect("mercurial command failed")
}
