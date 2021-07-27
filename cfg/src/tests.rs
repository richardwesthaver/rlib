//use tempfile::NamedTempFile;
use crate::config::repo::RepoType;
use crate::{NetworkConfig, PackageConfig, Result};

#[test]
fn pkg_cfg() {
  let pkg: PackageConfig = ron::from_str(
    r#"(name: "emacs",
        path: "contrib/bin/emacs",
        ops: [
          ("pull", "git pull"),
          ("install", "./configure && make install")])
     "#,
  )
  .unwrap();

  let package = PackageConfig::default();
}

#[test]
fn net_cfg() {
  let net: NetworkConfig = ron::from_str(
    r#"(socket: "127.0.0.1:0",
        transport: "udp-server",
        tunnel: Some("wireguard"),
        engine: Some("dmcodec-full"),
        peers:  None,
     )"#,
  )
  .unwrap();

  let network = NetworkConfig::default();
}

#[test]
fn repo_type() {
  assert_eq!(RepoType::GitRepository.to_string(), "git");
  assert_eq!(RepoType::MercurialRepository.to_string(), "hg");
}
