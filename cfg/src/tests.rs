//! cfg::tests
//use tempfile::NamedTempFile;
use crate::config::repo::{RepoConfig, RepoType};
use crate::{HgwebConfig, NetworkConfig, PackageConfig};
use std::path::PathBuf;
#[test]
fn test_package_cfg() {
  let mut pkg: PackageConfig = ron::from_str(
    r#"(name: "test-pack-cfg",
        repo: None,
        program: None,
        library: None)"#,
  )
  .unwrap();
  assert_eq!(pkg, PackageConfig::new("test-pack-cfg"));
  pkg.repo = Some(RepoConfig::new());
  assert_eq!(RepoConfig::new(), pkg.repo.unwrap());
}

#[test]
fn test_network_cfg() {
  let mut net: NetworkConfig = ron::from_str(
    r#"(socket: "127.0.0.1:0",
        transport: "udp-client",
        tunnel: None,
        engine: None,
        peers:  None)"#,
  )
  .unwrap();
  assert_eq!(net, NetworkConfig::default());
  net.socket = "0.0.0.0:0".parse().unwrap();
  assert_eq!(net.socket, "0.0.0.0:0".parse().unwrap());
  assert_ne!(net, NetworkConfig::default());
}

#[test]
fn test_repo_type() {
  assert_eq!(RepoType::GitRepository.to_string(), "git");
  assert_eq!(RepoType::MercurialRepository.to_string(), "hg");
}

#[test]
fn test_repo_config() {
  assert_eq!(RepoConfig::default().vcs, "hg");
}

#[test]
fn test_hgweb_insert() {
  let mut web_conf = HgwebConfig::default();

  web_conf
    .paths
    .insert(PathBuf::from("foo"), PathBuf::from("bar"));

  let wc2 = web_conf.paths.try_insert(
    PathBuf::from("contrib/lib/rust/tempdir"),
    PathBuf::from("contrib/lib/rust/tempdir"),
  );

  assert!(wc2.is_ok());
}
