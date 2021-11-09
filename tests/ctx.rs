//! ctx integration tests
use ctx::CtxInit;

#[tokio::test]
fn tokio_test() {}

#[tokio::test]
async fn tokio_async_test() {
  async fn helper() {}
  helper().await;
}

#[test]
fn tokio_main() {
  #[tokio::main]
  fn main() {}
  main();
}

mod submodule {
  #[tokio::main]
  fn main() {}
}

#[test]
fn tokio_main_with_disable_signals_sigterm_only() {
  #[tokio::main(disable_fatal_signals = sigterm_only)]
  fn main() {}
  main();
}

#[test]
fn tokio_main_with_disable_signals_none() {
  #[tokio::main(disable_fatal_signals = none)]
  fn main() {}
  main();
}

#[test]
fn tokio_main_with_disable_signals_all() {
  #[tokio::main(disable_fatal_signals = all)]
  fn main() {}
  main();
}

#[test]
fn tokio_main_with_disable_signals_default() {
  #[tokio::main(disable_fatal_signals = default)]
  fn main() {}
  main();
}
