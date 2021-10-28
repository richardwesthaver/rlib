//! lib.rs --- Objective types
#![feature(map_try_insert)]
#![feature(derive_default_enum)]
pub use ron;

mod id;
mod coll;
mod err;

mod config;
mod object;

pub use id::{NameSpace, Domain, Id};
pub use coll::{Coll, Collection};
pub use err::{Error, Result};

#[cfg(feature = "org")]
pub use object::doc::org::Org;

pub use object::{
  color::Color,
  location::{Point, City},
  direction::{CardinalDirection, RelativeDirection, EdgeDirection},
  doc::{Doc, DocExtension},
  media::{Media, MediaExtension},
  meta::{Meta, Property, Summary, Note},
  temperature::Temperature,
};

#[cfg(feature = "oauth")]
pub use config::auth::Oauth2Config;
#[cfg(feature = "git")]
pub use config::repo::git::GitRepository;
#[cfg(feature = "hg")]
pub use config::repo::hg::{export_hg_git, HgSubFile, HgwebConfig, MercurialConfig};

pub use config::{
  auth::{AuthConfig, SshConfig},
  database::DatabaseConfig,
  display::DisplayConfig,
  library::LibraryConfig,
  meta::MetaConfig,
  network::NetworkConfig,
  package::PackageConfig,
  program::ProgramConfig,
  project::ProjectConfig,
  registry::RegistryConfig,
  repo::RepoConfig,
  user::{UserConfig, TmuxSessionConfig, TmuxWindowConfig, TmuxPaneConfig, ShellConfig},
  
};

use ron::extensions::Extensions;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use std::io;

#[macro_export]
macro_rules! impl_config {
  ($($t:ident),*) => {
    $(
    impl Objective for $t {}
    impl Configure for $t {}
    )*
  };
}

impl_config!(
  MetaConfig,
  RepoConfig,
  DatabaseConfig,
  DisplayConfig,
  UserConfig,
  ShellConfig,
  TmuxSessionConfig,
  TmuxWindowConfig,
  TmuxPaneConfig,
  LibraryConfig,
  ProgramConfig,
  ProjectConfig,
  NetworkConfig,
  AuthConfig,
  SshConfig
);

#[cfg(feature = "oauth")]
impl_config!(Oauth2Config);

/// common trait for all config modules. This trait provides functions
/// for de/serializing to/from RON, updating fields, and formatting.
pub trait Configure: Objective {
  fn update(&self) -> Result<()> {
    Ok(())
  }
}

/// Objective trait
///
/// Defines Object behaviors, implemented by Objects
pub trait Objective {
  fn encode(&self) -> Result<Vec<u8>>
  where
    Self: Serialize,
  {
    Ok(bincode::serialize(self)?)
  }

  fn encode_into<W>(&self, writer: W) -> Result<()>
  where
    W: io::Write,
    Self: Serialize,
  {
    Ok(bincode::serialize_into(writer, self)?)
  }

  fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Self>
  where
    Self: Deserialize<'a>,
  {
    Ok(bincode::deserialize(bytes)?)
  }

  fn decode_from<R>(&self, rdr: R) -> Result<Self>
  where
    R: io::Read,
    Self: DeserializeOwned,
  {
    Ok(bincode::deserialize_from(rdr)?)
  }

  fn to_ron_writer<W>(&self, writer: W) -> Result<()>
  where
    W: io::Write,
    Self: Serialize,
  {
    Ok(ron::ser::to_writer_pretty(
      writer,
      &self,
      ron::ser::PrettyConfig::new()
        .with_indentor("  ".to_owned())
        .with_extensions(Extensions::all()),
    )?)
  }

  fn to_ron_string(&self) -> Result<String>
  where
    Self: Serialize,
  {
    Ok(ron::ser::to_string_pretty(
      &self,
      ron::ser::PrettyConfig::new().with_indentor("  ".to_owned()),
    )?)
  }

  fn from_ron_reader<R>(&self, mut rdr: R) -> Result<Self>
  where
    R: io::Read,
    Self: DeserializeOwned,
  {
    let mut bytes = Vec::new();
    rdr.read_to_end(&mut bytes)?;
    Ok(ron::de::from_bytes(&bytes)?)
  }

  fn from_ron_str<'a>(s: &'a str) -> Result<Self>
  where
    Self: Deserialize<'a>,
  {
    Ok(ron::de::from_bytes(s.as_bytes())?)
  }

  fn to_json_writer<W>(&self, writer: W) -> Result<()>
  where
    W: io::Write,
    Self: Serialize,
  {
    //    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    Ok(serde_json::ser::to_writer_pretty(writer, &self)?)
  }

  fn to_json_string(&self) -> Result<String>
  where
    Self: Serialize,
  {
    Ok(serde_json::ser::to_string_pretty(&self)?)
  }

  fn from_json_reader<R>(&self, mut rdr: R) -> Result<Self>
  where
    R: io::Read,
    Self: DeserializeOwned,
  {
    let mut bytes = Vec::new();
    rdr.read_to_end(&mut bytes)?;
    Ok(serde_json::de::from_slice(&bytes)?)
  }

  fn from_json_str<'a>(s: &'a str) -> Result<Self>
  where
    Self: Deserialize<'a>,
  {
    Ok(serde_json::de::from_slice(s.as_bytes())?)
  }
}
