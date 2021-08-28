//! obj::tests

use crate::{Doc, DocExtension, Media, MediaExtension, Objective, Org, Point};
use std::fs;
/// test Point object
use std::str::FromStr;

#[test]
fn test_location_points() {
  let pnt = Point::new(1.0, 2.0);
  assert!(String::from_str("(\n  lat: 1,\n  lng: 2,\n)").unwrap() == pnt.to_ron_string().unwrap());
  assert!(Point::from_ron_str("(lat: 1.0, lng: 2.0)").unwrap() == pnt);
}

/// test file metadata
#[test]
fn test_basic_file_metadata() {
  let attr = fs::metadata("Cargo.toml").unwrap();
  println!("{:?}", attr);
}

#[test]
fn test_docs() {
  let doc = Doc::default();
  assert!(doc == Doc::new("org"));
  assert!(doc.extension == DocExtension::from_str("org").unwrap());
}

#[test]
fn test_media() {
  let media = Media::new("wav");
  assert!(media.extension == MediaExtension::from_str("wav").unwrap());
}
/// Test Org parser functionality
#[test]
fn org_docs() {
  let org = Org::new();

  assert_eq!(org.contents, "");
  // let ron_org = r#"(
  // meta: (
  //   id: 0,
  //   tags: None,
  //   properties: None,
  //   ),
  //   created: "1970-01-01T00:00:00Z",
  //   updated: "1970-01-01T00:00:00Z",
  // ),
  // content: "",
  // notes: None,
  // )"#;
  assert!(org.encode().is_ok());
  let org = org.append("* hello world").unwrap();
  assert_eq!("* hello world", org.content());
}
