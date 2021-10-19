//! cli module
pub use clap::{App, AppSettings, Arg, ArgGroup, ArgSettings, Subcommand, ValueHint, ArgMatches, ColorChoice};
#[cfg(feature = "bs")]
pub mod comp_gen {
  pub use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
  pub use clap_generate::{generate, generate_to, Generator};
}
pub use indicatif;
pub use terminal_clipboard;
