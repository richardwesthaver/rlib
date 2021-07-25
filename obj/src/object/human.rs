use chrono::{DateTime, Utc};

use super::{Deserialize, Objective, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Default)]
pub struct Person {
  name: String,
  email: String,
  phone: u64,
  home: String,
  work: String,
  born: DateTime<Utc>,
}

impl Objective for Person {}
