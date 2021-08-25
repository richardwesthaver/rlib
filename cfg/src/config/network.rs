//! cfg::config::network
//!
//! Network configuration primitives

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use super::Configure;

/// Network configuration
#[derive(Serialize, Deserialize, Hash, Debug, PartialEq)]
pub struct NetworkConfig {
  /// a socket to bind
  pub socket: SocketAddr,
  /// transport to use (TCP/UDP/UNIX)
  pub transport: String,
  /// tunnel to use
  pub tunnel: Option<String>,
  /// network engine to attach
  pub engine: Option<String>,
  /// peers to register for O-RTT comms
  pub peers: Option<Vec<(String, String)>>,
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

impl Configure for NetworkConfig {}
