use std::{fmt, io};

use serde::{de, ser};

/// cfg Error type
pub enum Error {
  Io(io::Error),
  Ron(ron::error::Error),
  Serde(String),
  Hg(hg_parser::ErrorKind),
}

impl ser::Error for Error {
  fn custom<T: fmt::Display>(msg: T) -> Self {
    Error::Serde(msg.to_string())
  }
}

impl de::Error for Error {
  fn custom<T: fmt::Display>(msg: T) -> Self {
    Error::Serde(msg.to_string())
  }
}

impl From<hg_parser::ErrorKind> for Error {
  fn from(e: hg_parser::ErrorKind) -> Self {
    Error::Hg(e)
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::Io(e)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "IO error: {}", err),
      Error::Ron(ref err) => write!(f, "Ron parsing error: {}", err),
      Error::Serde(ref msg) => f.write_str(msg),
      Error::Hg(ref err) => write!(f, "lib::cfg MercurialRepo error: {}", err),
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "IO error: {}", err),
      Error::Ron(ref err) => write!(f, "Ron parsing error: {}", err),
      Error::Serde(ref msg) => f.write_str(msg),
      Error::Hg(ref err) => write!(f, "lib::cfg MercurialRepo error: {}", err),
    }
  }
}

impl std::error::Error for Error {}
