//! cfg/src/lib.rs -- Configuration Primitives for distributed systems
#![feature(map_try_insert)]
#![allow(dead_code)]

mod config;
mod de;
mod err;
mod ser;

/// Re-exports
pub use ron;

pub use crate::{
  config::{
    database::DatabaseConfig,
    display::DisplayConfig,
    library::LibraryConfig,
    network::NetworkConfig,
    package::PackageConfig,
    program::ProgramConfig,
    registry::RegistryConfig,
    repo,
    system::{SysConfig, SystemConfig, VmConfig, VmmConfig},
    user::UserConfig,
  },
  err::Error,
};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
