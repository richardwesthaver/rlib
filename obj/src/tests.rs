//! obj::tests

/// test Point object
#[test]
fn points() {
  use crate::object::Point;
  Point::new(1.0, 2.0);
}

/// test file metadata
#[test]
fn file_metadata() {
  use std::fs;
  let attr = fs::metadata("Cargo.toml").unwrap();
  println!("{:?}", attr);
}

/// Test Org parser functionality
#[test]
fn org_docs() {
  use crate::{Objective, Org};
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
