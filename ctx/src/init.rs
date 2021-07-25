use futures::Future;

pub fn tokio_test<F>(f: F) -> <F as Future>::Output
where
  F: Future,
{
  tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(f)
}

pub fn tokio_main<F>(f: F) -> <F as Future>::Output
where
  F: Future,
{
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(f)
}
