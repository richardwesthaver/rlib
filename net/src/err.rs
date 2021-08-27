//! net errors
use std::{fmt, io};

/// net Result type
pub type Result<T> = std::result::Result<T, Error>;

/// net Error type
pub enum Error {
  Io(io::Error),
  #[cfg(feature = "client")]
  Reqwest(reqwest::Error),
  Json(serde_json::Error),
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::Io(e)
  }
}

#[cfg(feature = "client")]
impl From<reqwest::Error> for Error {
  fn from(e: reqwest::Error) -> Self {
    Error::Reqwest(e)
  }
}

impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Error::Json(e)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "net IO error: {}", err),
      #[cfg(feature = "client")]
      Error::Reqwest(ref err) => write!(f, "net::client Reqwest error: {}", err),
      Error::Json(ref err) => write!(f, "net Json error: {}", err),
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "net IO error: {}", err),
      #[cfg(feature = "client")]
      Error::Reqwest(ref err) => write!(f, "net::client Reqwest error: {}", err),
      Error::Json(ref err) => write!(f, "net Json error: {}", err),
    }
  }
}

impl std::error::Error for Error {}
