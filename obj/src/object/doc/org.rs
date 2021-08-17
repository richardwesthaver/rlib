//! obj::doc::org
//!
//! Org document object types

use std::fs;
use std::path::PathBuf;

//use orgize::{
//  Element, Org as Organ, ParseConfig,
//};
use ron::{
  extensions::Extensions,
  ser::{to_string_pretty, PrettyConfig},
};
use logger::log::info;

use crate::Result;
use crate::object::{Deserialize, Meta, Note, Objective, Serialize, Property};

/// Org object type
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Org {
  meta: Meta,
  properties: Option<Vec<Property>>,
  contents: String,
  notes: Option<Vec<Note>>,
}

impl Org {
  /// Create a new Org object
  pub fn new() -> Self {
    Org {
      meta: Meta::default(),
      properties: None,
      contents: "".to_string(),
      notes: None,
    }
  }
  /// Create a new Org object from an Org-mode file
  pub fn from_file(path: &str) -> Result<Self> {
    let contents =
      String::from_utf8(fs::read(PathBuf::from(path))?).expect("failed to read utf8 string");
    let mut org = Org::new();
    org.contents = contents;
    info!("parsed org-file: {:?}", &path);
    info!(
      "{}",
      to_string_pretty(
        &org.meta,
        PrettyConfig::new()
          .with_indentor("  ".to_owned())
          .with_extensions(Extensions::all())
      )
      .unwrap()
    );
    Ok(org)
  }

  /// Append structured data to this Org document
  pub fn append(self, input: &str) -> Result<Self> {
    let mut doc = self;
    doc.contents += input;

    Ok(doc)
  }

  /// Return document metadata
  pub fn meta(&self) -> &Meta {
    &self.meta
  }

  /// Return document content
  pub fn content(&self) -> &str {
    &self.contents
  }

  /// Return notes linked to this object
  pub fn notes(&self) -> Option<&Vec<Note>> {
    if self.notes.is_none() {
      None
    } else {
      self.notes.as_ref()
    }
  }
}

impl Objective for Org {}

/// Test Org parser functionality
#[test]
fn org_docs() {
  use crate::{Objective, Org};
  let org = Org::new();
  assert_eq!(org.contents, "");

  let org_file = Org::from_file("doc.org");
  assert!(org_file.is_ok());

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
  let org = org.append("hello world").unwrap();
  assert_eq!("hello world", org.content());
}
