#[cfg(feature = "quic")]
pub mod quic;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "dns")]
pub mod dns;

pub enum EngineType {
  Quic,
  Http,
  Dns,
}
