#[test]
fn uris() {
  use crate::ObjectId;
}

#[test]
fn file_metadata() {
  use std::fs;
  let attr = fs::metadata("Cargo.toml").unwrap();
  println!("{:?}", attr);
}

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
