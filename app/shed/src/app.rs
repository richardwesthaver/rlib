use std::fmt;
use std::result::Result;

use cfg::NetworkConfig;
use db::Registry;

pub struct Client {
  engine: net::Client,
  registry: Registry,
  config: NetworkConfig,
}

impl Client {
  pub fn get(&self, key: String) -> Result<String, Err::Client> {
    Ok("heythere".to_string())
  }
}
pub struct Server {
  engine: net::Server,
  registry: Registry,
  config: NetworkConfig,
}

impl Server {}
