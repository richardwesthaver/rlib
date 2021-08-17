//! obj::doc
//!
//! Document object types
mod org;

pub use org::Org;

/// Doc enum type
pub enum Doc {
  Org,
  Pdf,
  Latex,
  Ascii,
  Html,
  Markdown,
}
