use std::{fmt, io};

pub enum Error {
  Io(io::Error),
  Cfg(cfg::Error),
  Logger(logger::Error),
  Repl(rustyline::error::ReadlineError),
  MidiInit(midir::InitError),
  MidiConnect(midir::ConnectError<midir::ConnectErrorKind>),
  MidiPortInfo(midir::PortInfoError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "rlib::kala IO error: {}", err),
      Error::Cfg(ref err) => write!(f, "rlib::kala Cfg error: {}", err),
      Error::Logger(ref err) => write!(f, "rlib::kala Logger error: {}", err),
      Error::Repl(ref err) => write!(f, "rlib::kala REPL error: {}", err),
      Error::MidiInit(ref err) => write!(f, "rlib::kala Midi Initialization error: {}", err),
      Error::MidiConnect(ref err) => write!(f, "rlib::kala Midi Connection error: {}", err),
      Error::MidiPortInfo(ref err) => write!(f, "rlib::kala Midi PortInfo error: {}", err),
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "rlib::kala IO error: {}", err),
      Error::Cfg(ref err) => write!(f, "rlib::kala Cfg error: {}", err),
      Error::Logger(ref err) => write!(f, "rlib::kala Logger error: {}", err),
      Error::Repl(ref err) => write!(f, "rlib::kala REPL error: {}", err),
      Error::MidiInit(ref err) => write!(f, "rlib::kala Midi Initialization error: {}", err),
      Error::MidiConnect(ref err) => write!(f, "rlib::kala Midi Connection error: {}", err),
      Error::MidiPortInfo(ref err) => write!(f, "rlib::kala Midi PortInfo error: {}", err),
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

impl From<midir::InitError> for Error {
  fn from(e: midir::InitError) -> Self {
    Error::MidiInit(e)
  }
}

impl From<midir::ConnectError<midir::ConnectErrorKind>> for Error {
  fn from(e: midir::ConnectError<midir::ConnectErrorKind>) -> Self {
    Error::MidiConnect(e)
  }
}

impl From<midir::PortInfoError> for Error {
  fn from(e: midir::PortInfoError) -> Self {
    Error::MidiPortInfo(e)
  }
}

impl std::error::Error for Error {}
