//! obj
//!
//! Object-oriented type system

mod err;

pub mod object;
pub mod paths;
pub use crate::{
  err::Error,
  object::{Doc, Meta, Note, Identity, Objective, Property, doc::Org},
};

/// obj Result wrapper
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;

