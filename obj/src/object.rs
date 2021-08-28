//! obj::object
//!
//! Concrete object types and traits. All type definitions conform to
//! the Serde spec.
mod color;
mod doc;
mod location;
mod media;
mod meta;
mod person;

pub use self::{
  color::Color,
  doc::{Doc, DocExtension, Org},
  location::{City, Point},
  media::{Media, MediaExtension},
  meta::{Meta, Note, Property, Summary},
  person::Person,
};

use std::io;

use hash::Id;
use serde::de::DeserializeOwned;
pub(crate) use serde::{Deserialize, Serialize};

use crate::Result;

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

/// Identity trait
///
/// Defines Identity-related behaviors, implemented by Objects
pub trait Identity: Sized + Objective {
  /// return the hashed bytes of an ObjectId
  fn id(&self) -> Id;
  /// return the hexcode string of an ObjectId
  fn hex_id(&self) -> String;
  /// return the human-readable string of an ObjectId
  fn string_id(&self) -> String;
  /// return the namespace of an ObjectId
  fn namespace_id(&self) -> String;
}
