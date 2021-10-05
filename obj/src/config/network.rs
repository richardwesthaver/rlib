//! cfg::config::network
//!
//! Network configuration primitives
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;

#[cfg(feature = "oauth")]
use yup_oauth2::ApplicationSecret;

/// Network configuration
#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Clone)]
pub struct NetworkConfig {
  /// a socket to bind
  pub socket: SocketAddr,
  /// tunnel to use
  pub tunnel: Option<String>,
  /// network engine to attach
  pub engine: EngineType,
  /// peers to register AOT
  pub peers: Option<Vec<(String, String)>>,
}

impl Default for NetworkConfig {
  fn default() -> Self {
    NetworkConfig {
      socket: "127.0.0.1:0".parse().unwrap(),
      tunnel: None,
      engine: EngineType::default(),
      peers: None,
    }
  }
}

#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Clone)]
pub enum EngineType {
  Quic,
  Http,
  Dns,
  Ssh,
  Uds,
}

impl Default for EngineType {
  fn default() -> Self {
    Self::Http
  }
}

impl std::fmt::Display for EngineType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      EngineType::Quic => write!(f, "quic"),
      EngineType::Http => write!(f, "http"),
      EngineType::Dns => write!(f, "dns"),
      EngineType::Ssh => write!(f, "ssh"),
      EngineType::Uds => write!(f, "uds"),
    }
  }
}

#[cfg(feature = "oauth")]
#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Clone, Default)]
pub struct Oauth2Config {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uris: Vec<String>,
  pub auth_uri: String,
  pub token_uri: String,
  pub project_id: Option<String>, //for apptoken
  pub client_email: Option<String>,
}

#[cfg(feature = "oauth")]
impl From<ApplicationSecret> for Oauth2Config {
  fn from(shh: ApplicationSecret) -> Self {
    Oauth2Config {
      client_id: shh.client_id,
      client_secret: shh.client_secret,
      redirect_uris: shh.redirect_uris,
      auth_uri: shh.auth_uri,
      token_uri: shh.token_uri,
      project_id: shh.project_id,
      client_email: shh.client_email,
    }
  }
}

#[cfg(feature = "oauth")]
impl From<Oauth2Config> for ApplicationSecret {
  fn from(cfg: Oauth2Config) -> Self {
    ApplicationSecret {
      client_id: cfg.client_id,
      client_secret: cfg.client_secret,
      redirect_uris: cfg.redirect_uris,
      auth_uri: cfg.auth_uri,
      token_uri: cfg.token_uri,
      project_id: cfg.project_id,
      client_email: cfg.client_email,
      ..Self::default()
    }
  }
}
