//! cfg::config::network
//!
//! Network configuration primitives

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Network configuration
#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct NetworkConfig {
  /// a socket to bind
  socket: SocketAddr,
  /// transport to use (TCP/UDP/UNIX)
  transport: String,
  /// tunnel to use
  tunnel: Option<String>,
  /// network engine to attach
  engine: Option<String>,
  /// peers to register for O-RTT comms
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
