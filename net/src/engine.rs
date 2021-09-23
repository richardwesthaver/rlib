#[cfg(feature = "dns")]
pub mod dns;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "quic")]
pub mod quic;

pub enum EngineType {
  Quic,
  Http,
  Dns,
}
