//! cfg::config
//!
//! Primitive configuration types

pub mod database;
pub mod display;
pub mod library;
pub mod network;
pub mod package;
pub mod program;
pub mod registry;
pub mod repo;
pub mod user;

use crate::Objective;
use crate::Result;

/// common trait for all config modules. This trait provides functions
/// for de/serializing to/from RON, updating fields, and formatting.
pub trait Configure: Objective {
  fn update(&self) -> Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_package_cfg() {
    let mut pkg: package::PackageConfig = ron::from_str(
      r#"(name: "test-pack-cfg",
        repo: None,
        program: None,
        library: None)"#,
    )
    .unwrap();
    assert_eq!(pkg, PackageConfig::new("test-pack-cfg"));
    pkg.repo = Some(repo::RepoConfigRepoConfig::new());
    assert_eq!(repo::RepoConfig::new(), pkg.repo.unwrap());
  }

  #[test]
  fn test_network_cfg() {
    let mut net: network::NetworkConfig = ron::from_str(
      r#"(socket: "127.0.0.1:0",
        transport: "udp",
        tunnel: None,
        engine: None,
        peers:  None)"#,
    )
    .unwrap();
    assert_eq!(net, network::NetworkConfig::default());
    net.socket = "0.0.0.0:0".parse().unwrap();
    assert_eq!(net.socket, "0.0.0.0:0".parse().unwrap());
    assert_ne!(net, network::NetworkConfig::default());
  }

  #[test]
  fn test_repo_type() {
    assert_eq!(repo::RepoType::GitRepository.to_string(), "git");
    assert_eq!(repo::RepoType::MercurialRepository.to_string(), "hg");
  }

  #[test]
  fn test_repo_config() {
    assert_eq!(repo::RepoConfig::default().vcs, "hg");
  }

  #[test]
  fn test_hgweb_insert() {
    let mut web_conf = repo::hg::HgwebConfig::default();

    web_conf
      .paths
      .insert(PathBuf::from("foo"), PathBuf::from("bar"));

    let wc2 = web_conf.paths.try_insert(
      PathBuf::from("contrib/lib/rust/tempdir"),
      PathBuf::from("contrib/lib/rust/tempdir"),
    );

    assert!(wc2.is_ok());
  }
}
