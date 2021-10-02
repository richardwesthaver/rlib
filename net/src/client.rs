//! network client primitives
pub use reqwest;

use obj::config::NetworkConfig;

pub struct Client {
  pub cfg: NetworkConfig,
}
