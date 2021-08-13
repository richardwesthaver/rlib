/// obj/src/object.rs --++--
/// Different types of external, concrete objects. All type
/// definitions conform to the Serde spec.
use std::io;

use chrono::{DateTime, Utc};
use hash::Id;
pub(crate) use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub(crate) use crate::Result;
mod color;
mod doc;
mod person;
mod location;
mod media;

pub use self::{
  color::Color,
  doc::Doc,
  person::Person,
  location::{City, Point},
  media::Media,
};

/// The Objective trait definition.
/// Used on types that derive (Serialize, Deserialize)
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

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Meta {
  id: u64,
  tags: String,
  properties: Properties,
  created: DateTime<Utc>,
  updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Properties {
  key: String,
  val: String,
}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Summary {
  meta: Meta,
  summary: String,
}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Note {
  meta: Meta,
  content: String,
}

pub trait Identity: Sized + Objective {
  fn id(&self) -> Id;
  fn update(&self) -> Self;
}
