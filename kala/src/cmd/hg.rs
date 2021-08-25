//! Mercurial command module
use cfg::repo::HgwebConfig;
use cmd_lib::{spawn_with_output, use_builtin_cmd, CmdResult};
use logger::log::info;
use std::io::BufRead;
use std::io::BufReader;

/// Given a `HgwebConfig` struct, start the hgweb server and log to
/// stdout.
pub fn hgweb(cfg: HgwebConfig) -> CmdResult {
  println!("found hgweb_config: {:?}", cfg);
  use_builtin_cmd!(echo, info);
  // Continuously process child outputs
  spawn_with_output!(hgweb)?.wait_with_pipe(&mut |pipe| {
    let cfg = option_env!("SHED_CFG").expect("$SHED_CFG undefined");
    info!("found config {}", cfg);
    BufReader::new(pipe)
      .lines()
      .filter_map(|line| line.ok())
      .take(10)
      .for_each(|line| info!("{}", line));
  });
  Ok(())
}
