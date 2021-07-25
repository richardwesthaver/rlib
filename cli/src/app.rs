mod cmd;
mod demo;
mod lab;
mod repl;
mod shed;
mod stash;

pub use clap::{App, AppSettings, SubCommand};

pub use self::{demo::DemoCli, lab::LabCli, repl::repl, shed::ShedCli};

pub type Cmd = clap::App<'static, 'static>;
pub type Arg = clap::Arg<'static, 'static>;
pub type ArgGroup = clap::ArgGroup<'static>;
