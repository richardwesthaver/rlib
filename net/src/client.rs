//! network client primitives
pub use reqwest;

use cfg::NetworkConfig;

pub struct Client {
  pub cfg: NetworkConfig,
}
