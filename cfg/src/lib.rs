//! cfg/src/lib.rs -- Configuration Primitives for distributed systems
mod config;
mod de;
mod err;
mod ser;

pub use crate::{
  config::{
    database::DatabaseConfig,
    lab::LabConfig,
    library::LibraryConfig,
    network::NetworkConfig,
    package::PackageConfig,
    program::ProgramConfig,
    registry::RegistryConfig,
    repo,
    shed::ShedConfig,
    store::StoreConfig,
    system::{SysConfig, SystemConfig, VmConfig, VmmConfig},
    user::UserConfig,
  },
  err::Error,
};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
