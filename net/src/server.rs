//! network server primitives
pub use hyper;
use obj::config::network::NetworkConfig;
use async_trait::async_trait;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Server {
  pub cfg: NetworkConfig,
}

#[async_trait]
pub trait Serve {
  async fn run(&self);
}

pub struct FileServer {
  socket: SocketAddr,
  path: PathBuf,
  registry: HashMap<String, String>,
}
