use std::{fs, io::Result};

use orgize::Org;
use ron::{
  extensions::Extensions,
  ser::{to_string_pretty, PrettyConfig},
};

/// iterate over the contents of a file, 'printing' the contents of
/// each org-element individually as a RON Value.
pub fn iter_file(file: &str) -> Result<()> {
  let contents = String::from_utf8(fs::read(file)?).unwrap();

  for event in Org::parse(&contents).iter() {
    println!("{:?}", event);
  }
  Ok(())
}

/// Print the contents of an Org file as RON.
pub fn print_ron(file: &str) -> Result<()> {
  let contents = String::from_utf8(fs::read(file)?).unwrap();

  println!(
    "{}",
    to_string_pretty(
      &Org::parse(&contents),
      PrettyConfig::new()
        .indentor("  ".to_owned())
        .extensions(Extensions::all()) // only used implicit some :(
    )
    .unwrap()
  );
  Ok(())
}
