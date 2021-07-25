use std::fs;
use std::path::{Path, PathBuf};

use orgize::{
  export::{DefaultHtmlHandler, HtmlHandler},
  Element, Org as Organ, ParseConfig,
};
use ron::{
  extensions::Extensions,
  ser::{to_string_pretty, PrettyConfig},
};
use sys::logger::log::{error, info};

use super::{Deserialize, Meta, Note, Objective, Result, Serialize};
use crate::err::Error;

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Org {
  meta: Meta,
  pub content: String,
  notes: Option<Vec<Note>>,
}

impl Org {
  pub fn new() -> Self {
    Org {
      meta: Meta::default(),
      content: "".to_string(),
      notes: None,
    }
  }

  pub fn from_file(&self, path: &str) -> Result<Self> {
    let contents =
      String::from_utf8(fs::read(PathBuf::from(path))?).expect("failed to read utf8 string");
    let mut org = Org::new();
    org.content = contents;
    info!("parsed org-file: {:?}", &path);
    info!(
      "{}",
      to_string_pretty(
        &org.meta,
        PrettyConfig::new()
          .indentor("  ".to_owned())
          .extensions(Extensions::all()) // only used implicit some :(
      )
      .unwrap()
    );
    Ok(org)
  }

  pub fn append(self, input: &str) -> Result<Self> {
    let mut doc = self;
    doc.content += input;

    Ok(doc)
  }

  pub fn meta(&self) -> &Meta {
    &self.meta
  }

  pub fn content(&self) -> &str {
    &self.content
  }

  pub fn notes(&self) -> Option<&Vec<Note>> {
    if self.notes.is_none() {
      None
    } else {
      self.notes.as_ref()
    }
  }
}

impl Objective for Org {}

pub struct OrgHtmlHandler(DefaultHtmlHandler);
