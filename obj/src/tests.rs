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

/// test related_paths
#[test]
fn related_paths() {
  use crate::paths::local_relative_path;
  use std::path::Path;
  local_relative_path(Path::new(".")).unwrap();
}
