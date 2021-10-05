use ctx::tokio::io::Result as CR;
use ctx::tokio::process::Command;
use std::process::Output;
//use std::process::Stdio;

#[macro_export]
macro_rules! impl_cmd {
  ($($t:ident),*) => {
    $(
      pub async fn $t(args: Vec<&str>) -> CR<Output> {
        Command::new("$t")
          .args(args.into_iter())
          .spawn()?
        .wait_with_output()
          .await
      }
    )*
  }
}

impl_cmd!(erl, dyalog, lua);

/// BQN interpreter
pub async fn bqn(args: Vec<&str>) -> CR<Output> {
  Command::new("bqn")
    .args(args.into_iter())
    .spawn()?
    .wait_with_output()
    .await
}

/// the ngn/k implementation (k6)
pub async fn k(args: Vec<&str>) -> CR<Output> {
  Command::new("k")
    .arg("/home/ellis/shed/bin/repl.k")
    .args(args.into_iter())
    .spawn()?
    .wait_with_output()
    .await
}

pub async fn k9(args: Vec<&str>) -> CR<Output> {
  Command::new("li2.0")
    .args(args.into_iter())
    .spawn()?
    .wait_with_output()
    .await
}

pub async fn gnu_apl(args: Vec<&str>) -> CR<Output> {
  Command::new("apl")
    .args(args.into_iter())
    .spawn()?
    .wait_with_output()
    .await
}
