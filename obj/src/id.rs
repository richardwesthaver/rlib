//! # Id types for different formats
//!
//! ULID is the preferred format.
pub use rusty_ulid::{self, Ulid};
use std::fmt;
use std::str::FromStr;
pub use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Id {
  ObjectId(u128),     // u128
  ConfigId(u32, u32), // u64
  UnitId(u32),        // u32
  AtomId(u16),        // u16
}

pub struct NameSpace {
  pub custom_id: Option<String>,
  pub id: Id,
  pub uri: Vec<Id>,
  pub next: Option<Id>,
}

pub struct Domain {
  pub ns: NameSpace,
  pub id: Id,
}

impl From<Uuid> for Id {
  fn from(uuid: Uuid) -> Self {
    Id::ObjectId(uuid.as_u128())
  }
}

impl From<Ulid> for Id {
  fn from(ulid: Ulid) -> Self {
    Id::ObjectId(u128::from(ulid))
  }
}

impl From<u128> for Id {
  fn from(src: u128) -> Self {
    Id::ObjectId(src)
  }
}

impl FromStr for Id {
  type Err = ();
  fn from_str(input: &str) -> Result<Id, Self::Err> {
    match input {
      i => Ok(Id::ObjectId(u128::from(Ulid::from_str(i).unwrap()))),
    }
  }
}

impl fmt::Display for Id {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Id::ObjectId(i) => write!(f, "{}", Ulid::from(i)),
      Id::ConfigId(i, t) => write!(f, "{}:{}", i, t),
      Id::UnitId(i) => write!(f, "{}", i),
      Id::AtomId(i) => write!(f, "{}", i),
    }
  }
}
