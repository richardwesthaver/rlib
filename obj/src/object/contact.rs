//! obj::person
//!
//! Object types for people

use chrono::{DateTime, Utc};

use super::{Deserialize, Objective, Serialize};

/// Person object type
///
/// Contains information about a specific person.
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Contact {
  name: String,
  email: String,
  phone: u64,
  home: String,
  work: String,
  born: DateTime<Utc>,
}

impl Objective for Contact {}
