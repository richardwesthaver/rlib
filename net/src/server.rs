//! network server primitives
pub use hyper;
use cfg::NetworkConfig;
use async_trait::async_trait;

pub struct Server {
  pub cfg: NetworkConfig,
}

#[async_trait]
pub trait Serve {
  async fn run(&self);
}
