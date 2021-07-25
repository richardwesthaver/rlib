use crate::app::{LabCli, ShedCli};
use crate::ctl::shell::emacs;
#[test]
fn shedcli_new() {
  ShedCli::new().run().unwrap();
}

#[test]
fn labcli_new() {
  LabCli::new().run().unwrap();
}

#[test]
fn emacs_spawn() {}
