//! util::path
//!
//! Helper functions for working with paths on filesystem
use std::path::{Component, Path, PathBuf};
use std::io::Error;
/// Given a path provided by the user, determines where generated files
/// related to that path should go.
pub fn local_relative_path(path: &Path) -> Result<PathBuf, Error> {
  let mut rel_path = PathBuf::new();
  for component in path.components() {
    match component {
      Component::Prefix(_) | Component::RootDir | Component::CurDir => {}
      Component::ParentDir => drop(rel_path.pop()), // noop if empty
      Component::Normal(name) => rel_path.push(name),
    }
  }

  Ok(rel_path)
}
