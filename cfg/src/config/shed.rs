//! config/shed --++-- Shed configuration
//!
//! Shed configurationn layer.
use std::{
  fs,
  io::Write,
  path::{Path, PathBuf},
};

use ron::{
  de::from_reader,
  extensions::Extensions,
  ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};
use sys::hash::{B3Hash, B3Hasher, Id};

use super::network::NetworkConfig;
use super::package::PackageConfig;
use crate::Result;

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct ShedConfig {
  id: String,
  shed_path: PathBuf,
  pkg_path: PathBuf,
  contrib_path: PathBuf,
  pkg_config: Option<Vec<(String, PackageConfig)>>,
  include: Option<PathBuf>,
  network: Option<NetworkConfig>,
}

impl Default for ShedConfig {
  // default params are relative
  fn default() -> Self {
    let id = Id::rand();
    let hash = id.state_hash(&mut B3Hasher::new());
    ShedConfig {
      id: hash.to_hex(),
      shed_path: PathBuf::from("~/shed"),
      pkg_path: PathBuf::from("pkg"),
      pkg_config: None,
      contrib_path: PathBuf::from("contrib"),
      include: None,
      network: Some(NetworkConfig::default()),
    }
  }
}

impl ShedConfig {
  pub fn new() -> Self {
    ShedConfig::default()
  }

  pub fn write(&self, path: &Path) -> Result<()> {
    let pretty = PrettyConfig::new()
      .with_indentor("  ".to_owned())
      .with_extensions(Extensions::all());
    let mut file = fs::File::create(path)?;
    let s = to_string_pretty(&self, pretty).expect("Serialization failed");
    write!(file, "{}", s).unwrap();
    println!("wrote to file - {}", path.display());
    Ok(())
  }

  pub fn load(path: &str) -> Result<Self> {
    let f = fs::File::open(path).expect("Failed to read config.ron file.");
    let config: ShedConfig = match from_reader(f) {
      Ok(x) => x,
      Err(e) => {
        println!("Failed to load config: {}", e);
        std::process::exit(1);
      }
    };
    Ok(config)
  }
  pub fn include(path: &str) -> Result<Self> {
    let f = fs::File::open(path).expect("Failed to read config.ron file.");
    let config: ShedConfig = match from_reader(f) {
      Ok(x) => x,
      Err(e) => {
        println!("Failed to include config: {}", e);
        std::process::exit(1);
      }
    };
    Ok(config)
  }
}
