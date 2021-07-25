//! config/network --++-- Network configuration.
//!
//! These structs are deserialized at runtime for any network-enabled component.
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
/// Userspace Network Configuration.
#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct NetworkConfig {
  socket: SocketAddr,
  transport: String,
  tunnel: Option<String>,
  engine: Option<String>,
  peers: Option<Vec<(String, String)>>,
}

impl Default for NetworkConfig {
  fn default() -> Self {
    NetworkConfig {
      socket: "127.0.0.1:0".parse().unwrap(),
      transport: "udp-client".to_string(),
      tunnel: None,
      engine: None,
      peers: None,
    }
  }
}
