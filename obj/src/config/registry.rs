//! cfg::config::registry
//!
//! Registry configuration primitives
//!
//! A Registry is just a list of key=val pairs wrapped in an API. The
//! RegistryConfig is used to bootstrap the Registry service, at which
//! point the client can submit commands to the service socket.
use super::Configure;
use super::{database::DatabaseConfig, network::NetworkConfig};
use crate::Objective;

use serde::{Deserialize, Serialize};
/// Registry configuration type
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq)]
pub struct RegistryConfig {
  leader: bool,
  key_type: String,
  val_type: String,
  crypto: Option<String>,
  net: Option<NetworkConfig>,
  db: DatabaseConfig,
}

impl Configure for RegistryConfig {}
impl Objective for RegistryConfig {}
