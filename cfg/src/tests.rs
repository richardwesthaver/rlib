use tempfile::NamedTempFile;

use crate::config::repo::RepoType;
use crate::{NetworkConfig, PackageConfig, Result, ShedConfig};

#[test]
fn shed() {
  let mut tmp = NamedTempFile::new().unwrap();
  let shed: ShedConfig = ron::from_str(
    r#"#![enable(implicit_some)]
(
  id: "12345",
  shed_path: ".",
  pkg_path: "pkg",
  contrib_path: "contrib",
)

"#,
  )
  .unwrap();

  let pkg: PackageConfig = ron::from_str(
    r#"(name: "emacs",
        path: "contrib/bin/emacs",
        ops: [
          ("pull", "git pull"),
          ("install", "./configure && make install")])
     "#,
  )
  .unwrap();

  let net: NetworkConfig = ron::from_str(
    r#"(socket: "127.0.0.1:0",
        transport: "udp-server",
        tunnel: Some("wireguard"),
        engine: Some("dmcodec-full"),
        peers:  None,
     )"#,
  )
  .unwrap();

  assert!(shed.write(tmp.path()).is_ok());
  assert!(ShedConfig::load(tmp.path().to_str().unwrap()).is_ok());

  let package = PackageConfig::default();
  let network = NetworkConfig::default();
}
#[test]
fn stash() {}
#[test]
fn repo_type() {
  assert_eq!(RepoType::GitRepository.to_string(), "git");
  assert_eq!(RepoType::MercurialRepository.to_string(), "hg");
}
