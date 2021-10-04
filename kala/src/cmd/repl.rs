use std::process::Output;

use ctx::tokio::process::Command;
//use std::process::Stdio;

/// BQN interpreter
pub async fn bqn(args: Vec<&str>) -> std::process::ExitStatus {
  Command::new("bqn")
    .args(args.into_iter())
    .spawn()
    .expect("BQN failed to start")
    .wait()
    .await
    .expect("BQN command failed")
}

/// Erlang runtime
pub async fn erl(args: Vec<&str>) -> Output {
  Command::new("erl")
    .args(args.into_iter())
    .spawn()
    .expect("Erlang failed to start")
    .wait_with_output()
    .await
    .expect("Erlang command failed")
}

/// the ngn/k implementation (k6)
pub async fn k(args: Vec<&str>) -> Output {
  Command::new("k")
    .arg("/home/ellis/shed/bin/repl.k")
    .args(args.into_iter())
    .spawn()
    .expect("ngn/k failed to start")
    .wait_with_output().await.expect("ngn/k command failed")
}

pub async fn k9(args: Vec<&str>) -> Output {
  Command::new("li2.0")
    .args(args.into_iter())
    .spawn().expect("shakti (k9) li2.0 failed to start")
    .wait_with_output().await.expect("shakti command failed")
}

pub async fn dyalog(args: Vec<&str>) -> Output {
  Command::new("dyalog")
    .args(args.into_iter())
    .spawn()
    .expect("dyalog failed to start")
    .wait_with_output()
    .await
    .expect("dyalog command failed")
}

pub async fn gnu_apl(args: Vec<&str>) -> Output {
  Command::new("apl")
    .args(args.into_iter())
    .spawn()
    .expect("dyalog failed to start")
    .wait_with_output()
    .await
    .expect("dyalog command failed")
}

pub async fn lua(args: Vec<&str>) -> Output {
  Command::new("lua")
    .args(args.into_iter())
    .spawn()
    .expect("lua failed to start")
    .wait_with_output()
    .await
    .expect("lua command failed")
}
