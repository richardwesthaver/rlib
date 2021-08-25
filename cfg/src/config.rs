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

use crate::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io;

/// common trait for all config modules. This trait provides functions
/// for de/serializing to/from RON, updating fields, and formatting.
///
/// TODO [2021-08-25 Wed 04:21] - consider adding json (with a feature
/// flag? ohhh..) and bincode
pub trait Configure {
  fn update(&self) -> Result<()> {
    Ok(())
  }
  fn to_ron_writer<W>(&self, writer: W) -> Result<()>
  where
    W: io::Write,
    Self: Serialize,
  {
    Ok(ron::ser::to_writer_pretty(
      writer,
      &self,
      ron::ser::PrettyConfig::new().with_indentor("  ".to_owned()),
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
}
