#[cfg(feature = "quic")]
pub mod quic;
#[cfg(feature = "http")]
pub mod http;

pub enum EngineType {
  Quic,
  Http,
}
