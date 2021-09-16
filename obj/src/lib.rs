//! obj
//!
//! Object-oriented type system
mod err;
mod object;
#[cfg(test)]
mod tests;

pub use err::{Error, Result};
pub use object::{
  City, Color, Doc, DocExtension, Identity, Media, MediaExtension, Meta, Note, Objective, Org,
  Contact, Point, Property, Summary,
};

// re-exports
pub use ron;
