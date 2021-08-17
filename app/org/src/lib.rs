//! Org functions and helpers
//!
//! This library relies on orgize, which hasn't been updated in some
//! time. We are likely to remove this crate and create a new
//! implementation in obj/object/doc/org.rs.
use std::{fs, io::Result};

use orgize::Org;
use ron::{
  ser::{to_string_pretty, PrettyConfig},
};

/// iterate over the contents of a file, 'printing' the contents of
/// each org-element individually as a Ron value.
pub fn iter_file(file: &str) -> Result<()> {
  let contents = String::from_utf8(fs::read(file)?).unwrap();

  for event in Org::parse(&contents).iter() {
    println!("{:?}", event);
  }
  Ok(())
}

/// Print the contents of an Org file as a Ron object.
pub fn print_ron(file: &str) -> Result<()> {
  let contents = String::from_utf8(fs::read(file)?).unwrap();

  println!(
    "{}",
    to_string_pretty(
      &Org::parse(&contents),
      PrettyConfig::new()
    )
    .unwrap()
  );
  Ok(())
}
