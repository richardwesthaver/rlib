//! obj::object
//!
//! Concrete object types and traits. All type definitions conform to
//! the Serde spec.
pub mod color;
pub mod doc;
pub mod location;
pub mod media;
pub mod meta;
pub mod contact;
pub mod temperature;
pub mod direction;

use crate::Result;
use std::collections::BTreeMap;
use hash::Id;

/// Identity trait
///
/// Defines Identity-related behaviors, implemented by Objects
pub trait Identity: Sized {
  /// return the hashed bytes of an ObjectId
  fn id(&self) -> Id;
  /// return the hexcode string of an ObjectId
  fn hex_id(&self) -> String;
  /// return the human-readable string of an ObjectId
  fn string_id(&self) -> String;
  /// return the namespace of an ObjectId
  fn namespace_id(&self) -> String;
}


/// Collection container trait for single-typed sets
pub struct Coll<T: Objective>(Vec<T>);

pub struct Collection<T: Objective>(BTreeMap<Id, T>);

#[cfg(test)]
mod test {
  use super::*;
  use std::fs;
  use std::str::FromStr;
  #[test]
  fn test_location_points() {
    let pnt = location::Point::new(1.0, 2.0);
    assert_eq!(
      String::from_str("(\n  lat: 1,\n  lng: 2,\n)").unwrap(),
      pnt.to_ron_string().unwrap()
    );
    assert_eq!(location::Point::from_ron_str("(lat: 1.0, lng: 2.0)").unwrap(), pnt);
  }

  /// test file metadata
  #[test]
  fn test_basic_file_metadata() {
    let attr = fs::metadata("Cargo.toml").unwrap();
    println!("{:?}", attr);
  }

  #[test]
  fn test_docs() {
    let doc = doc::Doc::default();
    //  assert_eq!(doc.extension(), DocExtension::from_str("org").unwrap());
  }

  #[test]
  fn test_media() {
    let media = media::Media::new("wav");
    assert_eq!(media.extension, media::MediaExtension::from_str("wav").unwrap());
  }

  /// Test Org parser functionality
  #[cfg(feature = "org")]
  #[test]
  fn test_org_docs() {
    let org = doc::Org::new();
    assert!(org.encode().is_ok());
  }
}
