use quinn::{TransportConfig, ClientConfig, ServerConfig};

pub struct Quic {
  transport: TransportConfig,
  client: Option<ClientConfig>,
  server: Option<ServerConfig>,
}

