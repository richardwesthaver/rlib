//! obj::media
//!
//! Media object types

use super::{Deserialize, Objective, Serialize};
use std::fmt;
use std::str::FromStr;

/// External Media types.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq)]
pub struct Media {
  pub extension: MediaExtension,
}

impl Media {
  pub fn new(ext: &str) -> Self {
    Media {
      extension: MediaExtension::from_str(ext).unwrap(),
    }
  }
}
impl Objective for Media {}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq)]
pub enum MediaExtension {
  Mp3,
  Wav,
  Png,
  Mp4,
  None,
}

impl fmt::Display for MediaExtension {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MediaExtension::Mp3 => write!(f, "mp3"),
      MediaExtension::Wav => write!(f, "wav"),
      MediaExtension::Png => write!(f, "png"),
      MediaExtension::Mp4 => write!(f, "mp4"),
      MediaExtension::None => write!(f, ""),
    }
  }
}

impl FromStr for MediaExtension {
  type Err = ();

  fn from_str(input: &str) -> Result<MediaExtension, Self::Err> {
    match input {
      "mp3" => Ok(MediaExtension::Mp3),
      "wav" => Ok(MediaExtension::Wav),
      "png" => Ok(MediaExtension::Png),
      "mp4" => Ok(MediaExtension::Mp4),
      "" => Ok(MediaExtension::None),
      _ => Err(()),
    }
  }
}
