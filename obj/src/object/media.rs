//! obj::media
//!
//! Media object types

use super::{Deserialize, Objective, Serialize};

/// External Media types.
#[derive(Serialize, Deserialize, Debug, Hash)]
pub enum Media {
  Video,
  Audio,
  Image,
}

impl Objective for Media {}
