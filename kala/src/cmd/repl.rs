//! cmd/repl.rs --- REPLs for programming languages
//!
//! note that all of these commands call some binary at runtime which
//! needs to be available on your path. These don't offer much
//! utility, but other embedded REPLs are available (see python module
//! in crate root)
use ctx::tokio::{io::Result as CR, process::Command};
use std::process::Output;

#[macro_export]
macro_rules! impl_cmd {
  ($($t:ident),*) => {
    $(
      pub async fn $t(args: Vec<&str>) -> CR<Output> {
        Command::new(stringify!($t))
          .args(args.into_iter())
          .spawn()?
        .wait_with_output()
          .await
      }
    )*
  }
}

impl_cmd!(erl, dyalog, lua, bqn, apl, k, shakti, python);
