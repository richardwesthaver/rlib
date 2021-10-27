//! rlib/obj/src/id.rs --- rlib::obj::id
//!
//! primitive ID types.

pub use rusty_ulid::{self, Ulid};
use std::fmt;
use std::str::FromStr;
pub use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Id {
  ObjectId(u128), // u128
  UnitId(u32),    // u32
  AtomId(u16),    // u16
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
  fn from_str(input: &str) -> std::result::Result<Id, Self::Err> {
    match input {
      i => Ok(Id::ObjectId(u128::from(Ulid::from_str(i).unwrap()))),
    }
  }
}

impl fmt::Display for Id {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Id::ObjectId(i) => {
        write!(f, "{}", Ulid::from(i))
      }
      Id::UnitId(i) => write!(f, "{}", i),
      Id::AtomId(i) => write!(f, "{}", i),
    }
  }
}
