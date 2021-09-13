//! cfg
//!
//! Configuration Primitives
#![feature(map_try_insert)]

pub mod config;
mod err;

pub use ron;

pub use crate::{
  config::{
    database::DatabaseConfig,
    display::{DisplayConfig, MonitorConfig},
    library::LibraryConfig,
    network::NetworkConfig,
    package::PackageConfig,
    program::ProgramConfig,
    registry::RegistryConfig,
    repo::{HgSubFile, HgwebConfig, RepoConfig, RepoType, SubRepo},
    user::UserConfig,
  },
  err::Error,
};

/// cfg Result wrapper
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
