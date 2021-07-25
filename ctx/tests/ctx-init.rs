use ctx::CtxInit;

#[ctx::test]
fn test_without_proof() {}

#[ctx::test]
fn test_with_proof(ctx: CtxInit) {
  println!("Got ctx: {:?}", ctx);
}

#[ctx::test]
async fn test_async_without_proof() {
  async fn helper() {}

  helper().await;
}

#[ctx::test]
async fn test_async_with_proof(ctx: CtxInit) {
  async fn helper(_ctx: CtxInit) {}

  helper(ctx).await;
}

#[test]
fn test_main_without_proof() {
  #[ctx::main]
  fn main() {}

  main();
}

#[test]
fn test_main_with_proof() {
  #[ctx::main]
  fn main(ctx: CtxInit) {
    println!("Got ctx: {:?}", ctx);
  }

  main();
}

mod submodule {
  #[ctx::main]
  fn main() {}
}

#[test]
fn test_main_with_disable_signals_sigterm_only() {
  #[ctx::main(disable_fatal_signals = sigterm_only)]
  fn main() {}

  main();
}

#[test]
fn test_main_with_disable_signals_none() {
  #[ctx::main(disable_fatal_signals = none)]
  fn main() {}

  main();
}

#[test]
fn test_main_with_disable_signals_all() {
  #[ctx::main(disable_fatal_signals = all)]
  fn main() {}

  main();
}

#[test]
fn test_main_with_disable_signals_default() {
  #[ctx::main(disable_fatal_signals = default)]
  fn main() {}

  main();
}
