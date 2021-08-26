//! obj::doc::org
//!
//! Org document object types
//!
//! The Org-mode format is particularly difficult to parse, but can be
//! done. The best resources for learning how to create a parser for
//! Org is available in the Org-Element API docs.
//!
//! Resources:
//! - http://ergoemacs.org/emacs/elisp_parse_org_mode.html
//! - https://orgmode.org/manual/Using-the-Mapping-API.html
//! - https://orgmode.org/manual/Using-the-Property-API.html
//! - https://orgmode.org/worg/dev/org-element-api.html
use std::fs;
use std::path::PathBuf;
use logger::log::info;
use ron::{
  extensions::Extensions,
  ser::{to_string_pretty, PrettyConfig},
};

use crate::object::{Deserialize, Meta, Note, Objective, Property, Serialize};
use crate::Result;

/// Org object type
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Org {
  pub meta: Meta,
  pub properties: Option<Vec<Property>>,
  pub contents: String,
  pub notes: Option<Vec<Note>>,
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
