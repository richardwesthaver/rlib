use std::{fmt, io};

pub enum Error {
  Io(io::Error),
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
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "IO error: {}", err),
    }
  }
}

impl std::error::Error for Error {}
