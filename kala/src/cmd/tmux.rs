//! cmd/tmux.rs --- tmux commands
use crate::Result;
use logger::log::error;
use obj::TmuxSessionConfig;
use obj::RelativeDirection::*;
pub use tmux_interface::{HasSession, NewSession, TargetSession, TmuxCommand};

pub fn tmux_attach_session(name: &str) -> Result<()> {
  TmuxCommand::new().attach_session().target_session(name).output().unwrap();
  Ok(())
}
pub fn tmux_kill_session(name: &str) -> Result<()> {
  TmuxCommand::new().kill_session().target_session(name).output().unwrap();
  Ok(())
}
/// Build a tmux session from `TmuxSessionConfig`
pub fn tmux_build_session(cfg: TmuxSessionConfig) -> Result<()> {
  if HasSession::new()
    .target_session(&cfg.name)
    .output()
    .unwrap()
    .0
    .status
    .success()
    == true
  {
    error!("Tmux session {} already exists", cfg.name);
  } else {
    // bootstrap detached session
    let mut tmux = TmuxCommand::new();
    tmux
      .new_session()
      .detached()
      .session_name(&cfg.name)
      .output()
      .unwrap();
    // build windows
    for i in cfg.windows.iter() {
      tmux.new_window().window_name(&i.name).output().unwrap();
      for p in i.panes.iter() {
        match &p.position {
          None => {
            tmux.shell_cmd(p.init.as_ref().unwrap()).output().unwrap();
          }
          Some(rp) => {
            let pinit = match rp {
              Up => tmux
                .shell_cmd(p.init.as_ref().unwrap())
                .split_window()
                .before()
                .vertical()
                .output(),
              Down => tmux
                .shell_cmd(p.init.as_ref().unwrap())
                .split_window()
                .vertical()
                .output(),
              Left => tmux
                .shell_cmd(p.init.as_ref().unwrap())
                .split_window()
                .before()
                .horizontal()
                .output(),
              Right => tmux
                .shell_cmd(p.init.as_ref().unwrap())
                .split_window()
                .horizontal()
                .output(),
            };
            pinit.unwrap();
          }
        }
      }
    }
  }
  Ok(())
}

#[test]
fn tmux_session() {
  let mut cfg = TmuxSessionConfig::default();
  cfg.name = "sesh".to_string();
  tmux_build_session(cfg).unwrap();
  tmux_kill_session("sesh").unwrap();
}
