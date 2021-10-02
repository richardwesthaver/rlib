//! network server primitives
use async_trait::async_trait;
pub use hyper;
use obj::config::NetworkConfig;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;

pub struct Server {
  pub cfg: NetworkConfig,
}

#[async_trait]
pub trait Serve {
  async fn run(&self);
}

pub struct FileServer {
  pub socket: SocketAddr,
  pub path: PathBuf,
  pub registry: HashMap<String, String>,
}
