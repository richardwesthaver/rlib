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
use logger::log::{error, info};

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

#[test]
fn org_docs() {
  use crate::{Objective, Org};
  let org = Org::new();
  assert_eq!(org.content, "");

  let org_file = org.from_file("doc.org");
  assert!(org_file.is_ok());

  let ron_org = r#"(
  meta: (
    id: 0,
    tags: "",
    properties: (
      key: "",
      val: "",
    ),
    created: "1970-01-01T00:00:00Z",
    updated: "1970-01-01T00:00:00Z",
  ),
  content: "",
  notes: None,
)"#;
  assert_eq!(org.to_ron_string().unwrap(), ron_org);

  assert!(org.encode().is_ok());
  let org_bytes = vec![
    0, 0, 0, 0, 20, 49, 57, 55, 48, 45, 48, 49, 45, 48, 49, 84, 48, 48, 58, 48, 48, 58, 48, 48, 90,
    20, 49, 57, 55, 48, 45, 48, 49, 45, 48, 49, 84, 48, 48, 58, 48, 48, 58, 48, 48, 90, 0, 0,
  ];
  assert_eq!(org.encode().unwrap(), org_bytes);

  let org = org.append("hello world").unwrap();
  assert_eq!("hello world", org.content());
}
