//! obj errors
use std::{fmt, io};

/// obj Result wrapper
pub type Result<T> = std::result::Result<T, Error>;

/// obj Error type
#[derive(Debug)]
pub enum Error {
  Message(String),
  Ron(ron::error::Error),
  Io(io::Error),
  Bincode(bincode::Error),
  Utf8(std::string::FromUtf8Error),
}

impl serde::ser::Error for Error {
  fn custom<T: fmt::Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl serde::de::Error for Error {
  fn custom<T: fmt::Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Message(msg) => f.write_str(msg),
      Error::Io(ref err) => write!(f, "obj IO error: {}", err),
      Error::Ron(ref err) => write!(f, "obj Ron error: {}", err),
      Error::Bincode(ref err) => write!(f, "obj Bincode error: {}", err),
      Error::Utf8(ref err) => write!(f, "obj Utf8 error: {}", err),
    }
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::Io(e)
  }
}

impl From<std::string::FromUtf8Error> for Error {
  fn from(err: std::string::FromUtf8Error) -> Self {
    Error::Utf8(err)
  }
}

impl From<ron::Error> for Error {
  fn from(e: ron::Error) -> Self {
    Error::Ron(e)
  }
}

impl From<bincode::Error> for Error {
  fn from(e: bincode::Error) -> Self {
    Error::Bincode(e)
  }
}

impl std::error::Error for Error {}
