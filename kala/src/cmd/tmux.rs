use logger::log::error;
use tmux_interface::{HasSession, TmuxCommand};

use crate::Result;

pub fn tmux_build_session(session_name: &str) -> Result<()> {
  if HasSession::new()
    .target_session(session_name)
    .output()
    .unwrap()
    .0
    .status
    .success()
    == true
  {
    error!("Tmux session {} already exists", session_name);
  } else {
    let tmux = TmuxCommand::new();
    tmux
      .new_session()
      .detached()
      .session_name(session_name)
      .window_name("status")
      .shell_command("htop")
      .output()
      .unwrap();
    tmux
      .split_window()
      .horizontal()
      .shell_command("systemd-cgtop")
      .output()
      .unwrap();
    tmux
      .split_window()
      .vertical()
      .shell_command("journalctl -xef")
      .output()
      .unwrap();
    tmux
      .new_window()
      .window_name("emacs")
      .shell_command("emacs -nw -l ~/.emacs.d/lisp/init-home-tmux.el")
      .output()
      .unwrap();

    tmux.attach_session().target_session(session_name).output().unwrap();
  }

  Ok(())
}
