//! Mercurial command module
use obj::config::repo::hg::HgwebConfig;
use cmd_lib::{spawn_with_output, use_builtin_cmd, CmdResult};
use logger::log::info;
use std::io::BufRead;
use std::io::BufReader;

/// Given a `HgwebConfig` struct, start the hgweb server and log to
/// stdout.
/// TODO: import old shed_multi_server
pub fn hgweb(cfg: &HgwebConfig) -> CmdResult {
  println!("found hgweb_config: {:?}", cfg);
  use_builtin_cmd!(echo, info);
  // Continuously process child outputs
  spawn_with_output!(hg serve)?.wait_with_pipe(&mut |pipe| {
    BufReader::new(pipe)
      .lines()
      .filter_map(|line| line.ok())
      .take(10)
      .for_each(|line| info!("{}", line));
  });
  Ok(())
}
