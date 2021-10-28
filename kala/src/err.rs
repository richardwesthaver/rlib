//! err.rs --- kala errors
use std::{fmt, io};

/// kala Result type
pub type Result<T> = std::result::Result<T, Error>;
pub type R<T, E> = std::result::Result<T, E>;
/// kala Error type
pub enum Error {
  Io(io::Error),
  Obj(obj::Error),
  Logger(logger::Error),
  #[cfg(feature = "midi")]
  MidiInit(midir::InitError),
  #[cfg(feature = "midi")]
  MidiConnect(midir::ConnectError<midir::ConnectErrorKind>),
  #[cfg(feature = "midi")]
  MidiPortInfo(midir::PortInfoError),
  #[cfg(feature = "input")]
  Readline(rustyline::error::ReadlineError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "rlib::kala IO error: {}", err),
      Error::Obj(ref err) => write!(f, "kala Obj error: {}", err),
      Error::Logger(ref err) => write!(f, "rlib::kala Logger error: {}", err),
      #[cfg(feature = "midi")]
      Error::MidiInit(ref err) => write!(f, "rlib::kala Midi Initialization error: {}", err),
      #[cfg(feature = "midi")]
      Error::MidiConnect(ref err) => write!(f, "rlib::kala Midi Connection error: {}", err),
      #[cfg(feature = "midi")]
      Error::MidiPortInfo(ref err) => write!(f, "rlib::kala Midi PortInfo error: {}", err),
      #[cfg(feature = "input")]
      Error::Readline(ref err) => write!(f, "rlib::kala Readline error: {}", err),
    }
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Io(ref err) => write!(f, "rlib::kala IO error: {}", err),
      Error::Obj(ref err) => write!(f, "kala Obj error: {}", err),
      Error::Logger(ref err) => write!(f, "rlib::kala Logger error: {}", err),
      #[cfg(feature = "midi")]
      Error::MidiInit(ref err) => write!(f, "rlib::kala Midi Initialization error: {}", err),
      #[cfg(feature = "midi")]
      Error::MidiConnect(ref err) => write!(f, "rlib::kala Midi Connection error: {}", err),
      #[cfg(feature = "midi")]
      Error::MidiPortInfo(ref err) => write!(f, "rlib::kala Midi PortInfo error: {}", err),
      #[cfg(feature = "input")]
      Error::Readline(ref err) => write!(f, "rlib::kala Readline error: {}", err),
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

impl From<obj::Error> for Error {
  fn from(e: obj::Error) -> Self {
    Error::Obj(e)
  }
}

#[cfg(feature = "midi")]
impl From<midir::InitError> for Error {
  fn from(e: midir::InitError) -> Self {
    Error::MidiInit(e)
  }
}

#[cfg(feature = "midi")]
impl From<midir::ConnectError<midir::ConnectErrorKind>> for Error {
  fn from(e: midir::ConnectError<midir::ConnectErrorKind>) -> Self {
    Error::MidiConnect(e)
  }
}

#[cfg(feature = "midi")]
impl From<midir::PortInfoError> for Error {
  fn from(e: midir::PortInfoError) -> Self {
    Error::MidiPortInfo(e)
  }
}

#[cfg(feature = "input")]
impl From<rustyline::error::ReadlineError> for Error {
  fn from(e: rustyline::error::ReadlineError) -> Self {
    Error::Readline(e)
  }
}

impl std::error::Error for Error {}
