//! Mercurial command module
use tokio::process::Command;
use logger::log::debug;
use obj::MercurialConfig;
use std::{io::Error, process::Output, result::Result};
/// Given a `HgwebConfig` struct, start the hgweb server and log to
/// stdout.
/// TODO: import old shed_multi_server
pub async fn hgweb(cfg: &MercurialConfig) -> Result<Output, Error> {
  debug!("found hgrc: {:?}", cfg);
  let output = Command::new("hg").arg("serve").output();
  output.await
}

pub async fn hg<'a>(args: &[&'a str]) -> Result<Output, Error> {
  Command::new("hg")
    .args(args.into_iter())
    .spawn()
    .expect("mercurial error")
    .wait_with_output()
    .await
}
