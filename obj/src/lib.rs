//! obj
//!
//! Object-oriented type system
mod err;

pub mod object;
pub use crate::{
  err::Error,
  object::{
    doc::Org,
    location::{City, Point},
    Doc, Identity, Meta, Note, Objective, Property,
  },
};

/// obj Result wrapper
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
