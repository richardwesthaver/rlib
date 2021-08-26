//! obj
//!
//! Object-oriented type system
mod err;

pub mod object;
pub use crate::{
  err::Error,
  object::{
    Objective, Identity,
    Color, Doc, City, Point, Media, Person,
    doc::Org,
  }
};

/// obj Result wrapper
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
