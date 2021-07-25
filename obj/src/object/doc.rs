mod org;

pub use org::Org;

use super::{Deserialize, Meta, Note, Objective, Result, Serialize};

pub enum Doc {
  Org,
}
