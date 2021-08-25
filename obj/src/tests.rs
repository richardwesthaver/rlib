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
