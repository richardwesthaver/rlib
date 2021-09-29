use ctx::tokio::process::Command;
//use std::process::Stdio;
pub async fn bqn() -> std::process::ExitStatus {
  Command::new("bqn")
    .arg("-r")
    .spawn()
    .expect("BQN failed to start")
    .wait()
    .await
    .expect("BQN command failed")
}

pub async fn erl() -> std::process::ExitStatus {
  Command::new("erl")
    .spawn()
    .expect("Erlang failed to start")
    .wait()
    .await
    .expect("Erlang command failed")
}

pub async fn k() -> std::process::ExitStatus {
  Command::new("k")
    .arg("/home/ellis/shed/bin/repl.k")
    .spawn()
    .expect("ngn/k failed to start")
    .wait()
    .await
    .expect("ngn/k command failed")
}

pub async fn dyalog() -> std::process::ExitStatus {
  Command::new("dyalog")
    .arg("-b")
    .spawn()
    .expect("dyalog failed to start")
    .wait()
    .await
    .expect("dyalog command failed")
}
