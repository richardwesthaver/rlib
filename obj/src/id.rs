//! # Id types for different formats
//!
//! ULID is the preferred format.
pub use rusty_ulid::{self, Ulid};
pub use uuid::Uuid;


pub enum Id {
  ObjectId(u128), // u128
  ConfigId(u32, u32), // u64
  UnitId(u32), // u32
  AtomId(u16), // u16
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
