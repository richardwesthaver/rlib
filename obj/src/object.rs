//! obj::object
//!
//! Concrete object types and traits. All type definitions conform to
//! the Serde spec.
use std::io;

use chrono::{DateTime, Utc};
use hash::Id;
use serde::de::DeserializeOwned;
pub(crate) use serde::{Deserialize, Serialize};

use crate::Result;
mod color;
pub mod doc;
mod person;
pub mod location;
mod media;

pub use self::{
  color::Color,
  doc::Doc,
  location::{City, Point},
  media::Media,
  person::Person,
};

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
/// Defines Id-related behaviors, implemented by Objects
pub trait Identity: Sized + Objective {
  fn id(&self) -> Id;
  fn update(&self) -> Self;
}

/// Meta object
///
/// This struct is built into other Objects, providing basic data for
/// static analysis.
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Meta {
  id: u64,
  tags: Option<String>,
  properties: Option<Vec<Property>>,
  created: DateTime<Utc>,
  updated: DateTime<Utc>,
}

impl Meta {
  pub fn new() -> Self {
    Meta {
      id: 0x00,
      tags: None,
      properties: None,
      created: Utc::now(),
      updated: Utc::now(),
    }
  }
}
impl Default for Meta {
  fn default() -> Self {
      Meta::new()
  }
}
/// Property object
///
/// An isolated property consisting of a (key,val) pair.
///
/// TODO <2021-08-17 Tue 01:28> - this should be a trait. make
/// metadata module for this.
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Property {
  key: String,
  val: String,
}

/// Summary object
///
/// A summary of an object.
///
/// TODO <2021-08-17 Tue 01:30> make this a trait too.
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Summary {
  meta: Meta,
  summary: String,
}

/// Note object
///
/// A note pertaining to one or many objects.
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Note {
  meta: Meta,
  content: String,
}
