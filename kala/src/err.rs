use std::{fmt, io};

pub enum Error {
  Io(io::Error),
  Cfg(cfg::Error),
  Logger(logger::Error),
  Repl(rustyline::error::ReadlineError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "lib::cli IO error: {}", err),
      Error::Cfg(ref err) => write!(f, "lib::cli Cfg error: {}", err),
      Error::Logger(ref err) => write!(f, "lib::cli Logger error: {}", err),
      Error::Repl(ref err) => write!(f, "lib::cli REPL error: {}", err),
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "lib::cli IO error: {}", err),
      Error::Cfg(ref err) => write!(f, "lib::cli Cfg error: {}", err),
      Error::Logger(ref err) => write!(f, "lib::cli Logger error: {}", err),
      Error::Repl(ref err) => write!(f, "lib::cli REPL error: {}", err),
    }
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::Io(e)
  }
}

impl From<logger::Error> for Error {
  fn from(e: logger::Error) -> Self {
    Error::Logger(e)
  }
}

impl From<cfg::Error> for Error {
  fn from(e: cfg::Error) -> Self {
    Error::Cfg(e)
  }
}

impl From<rustyline::error::ReadlineError> for Error {
  fn from(e: rustyline::error::ReadlineError) -> Self {
    Error::Repl(e)
  }
}

impl std::error::Error for Error {}
