//! network client primitives
pub use reqwest;

use obj::config::network::NetworkConfig;

pub struct Client {
  pub cfg: NetworkConfig,
}
