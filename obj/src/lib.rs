//! obj
//!
//! Object Types
#![feature(map_try_insert)]
#![feature(derive_default_enum)]
pub mod coll;
pub mod config;
pub mod id;
pub mod object;

mod err;

pub use err::{Error, Result};

// re-exports
pub use ron;

use ron::extensions::Extensions;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io;

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
