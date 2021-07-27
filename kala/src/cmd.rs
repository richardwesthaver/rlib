//! kalash::cmd
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;

use cfg::repo::HgWebConfig;
use cmd_lib::{run_cmd, spawn_with_output, use_builtin_cmd, CmdResult, FunResult};
use logger::log::info;

pub fn hgweb(cfg: HgWebConfig) -> CmdResult {
  use_builtin_cmd!(echo, info);

  // Continuously process child process' outputs
  spawn_with_output!(hgweb)?.wait_with_pipe(&mut |pipe| {
    let cfg = option_env!("SHED_CFG").expect("$SHED_CFG undefined");
    BufReader::new(pipe)
      .lines()
      .filter_map(|line| line.ok())
      .take(10)
      .for_each(|line| info!("{}", line));
  });
  Ok(())
}
