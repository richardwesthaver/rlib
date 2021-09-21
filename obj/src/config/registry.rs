//! cfg::config::registry
//!
//! Registry configuration primitives
//!
//! A Registry is just a list of key=val pairs wrapped in an API. The
//! RegistryConfig is used to bootstrap the Registry service, at which
//! point the client can submit commands to the service socket.
use super::network::NetworkConfig;
use super::Configure;
use serde::{Deserialize, Serialize};
/// Registry configuration type
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq)]
pub struct RegistryConfig {
  local: bool,
  key_type: String,
  val_type: String,
  crypto: Option<String>,
  net: Option<NetworkConfig>,
}

impl Configure for RegistryConfig {}
