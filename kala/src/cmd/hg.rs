//! Mercurial command module
use crate::err::R;
use std::io::Error as E;

use ctx::tokio::process::Command;
use logger::log::debug;
use obj::config::MercurialConfig;
use std::process::Output;

/// Given a `HgwebConfig` struct, start the hgweb server and log to
/// stdout.
/// TODO: import old shed_multi_server
pub async fn hgweb(cfg: &MercurialConfig) -> R<Output, E> {
  debug!("found hgrc: {:?}", cfg);
  let output = Command::new("hg").arg("serve").output();
  output.await
}

pub async fn hg<'a>(args: &[&'a str]) -> R<Output, E> {
  Command::new("hg")
    .args(args.into_iter())
    .spawn()
    .expect("mercurial error")
    .wait_with_output()
    .await
}
