use quinn::{ClientConfig, ServerConfig, TransportConfig};

pub struct Quic {
  transport: TransportConfig,
  client: Option<ClientConfig>,
  server: Option<ServerConfig>,
}
