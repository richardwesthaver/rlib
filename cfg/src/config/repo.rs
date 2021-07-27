//! config/repo --++-- Repository configuration
//!
//! Config structs for both Mercurial (primary) and Git.

use std::{
  collections::HashMap,
  fmt,
  fs::File,
  io::{LineWriter, Read, Write},
  net::SocketAddr,
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
};

use git2::Repository as GitRepository;
use hg_parser::MercurialRepository;
use serde::{Deserialize, Serialize};
use sys::logger::log::{info, trace};

use crate::Result;
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct RepoConfig {
  vcs: String,
  origin: String,
  path: PathBuf,
}

impl RepoConfig {
  pub fn new() -> Self {
    RepoConfig::default()
  }
}

impl Default for RepoConfig {
  fn default() -> Self {
    RepoConfig {
      vcs: "hg".to_string(),
      origin: "".to_string(),
      path: PathBuf::from("."),
    }
  }
}

pub enum RepoType {
  MercurialRepository,
  GitRepository,
}

impl fmt::Display for RepoType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RepoType::MercurialRepository => write!(f, "hg"),
      RepoType::GitRepository => write!(f, "git"),
    }
  }
}

pub struct SubRepo {
  vcs: String,
  origin: String,
  path: String,
}

pub struct HgSubFile {
  path: PathBuf,
  subrepos: Vec<SubRepo>,
}

impl HgSubFile {
  pub fn new() -> Self {
    HgSubFile {
      path: PathBuf::from(".hgsub"),
      subrepos: vec![],
    }
  }

  /// should perform validation to ensure that the path is a hg repo,
  /// and does in fact exist, but the .hgsub file doesn't need to.
  pub fn parent(mut self, path: impl AsRef<Path>) -> Result<Self> {
    self.path = path.as_ref().to_path_buf().join(".hgsub").canonicalize()?;
    Ok(self)
  }

  /// insert a subrepo into this HgSubFile. does not clone the source
  /// or ensure that path exists. Takes an optional argument of 'hg'
  /// or 'git' to indicate the subrepo-type. None represents a
  /// local-only repo.
  pub fn insert(&self, name: &str, vcs: Option<&str>) -> Result<()> {
    let root = &self.path;

    let subrepo = (name, vcs);

    Ok(())
  }

  // Sort the full contents of a .hgsub file alphabetically.
  // This makes things slightly easier to find in mono-repos.
  pub fn sort(self) -> Result<Self> {
    let mut fd = File::open(&self.path)?;
    let len = fd.metadata().unwrap().len() as usize;
    let mut bufr = String::with_capacity(len);
    fd.read_to_string(&mut bufr).unwrap(); // This reads the entire file into memory.
    drop(fd); // drop the old readonly file descriptor

    let mut subs = Vec::new();

    trace!("starting sort of {:?} lines", &self.path.canonicalize()?);
    for line in bufr.lines() {
      subs.push(line.as_bytes());
    }
    subs.sort_unstable();
    trace!("lines have been sorted");

    let file = File::create(&self.path)?;
    let mut file = LineWriter::new(file);
    for sub in subs.iter() {
      file.write_all(sub)?;
      file.write_all(b"\n")?;
    }

    file.flush()?;
    info!("sorted {:?}", &self.path.canonicalize()?);

    Ok(self)
  }
}

impl Default for HgSubFile {
  fn default() -> Self {
    HgSubFile {
      path: PathBuf::from(""),
      subrepos: vec![],
    }
  }
}

/// We don't remember the file path in this case because all HgWebConfig values
/// are relative to $PWD
#[derive(Serialize, Deserialize, Debug)]
pub struct HgWebConfig {
  name: String,
  contact: String,
  description: String,
  extensions: Vec<String>,
  sock: SocketAddr,
  paths: HashMap<PathBuf, PathBuf>,
}

impl HgWebConfig {
  pub fn new() -> Self {
    Self {
      name: "".to_string(),
      contact: "".to_string(),
      description: "".to_string(),
      extensions: vec![],
      sock: "0.0.0.0:0"
        .parse()
        .expect("could not parse hgweb socketaddr"),
      paths: HashMap::new(),
    }
  }

  pub fn from_path(path: &Path) -> Self {
    HgWebConfig::new()
  }

  pub fn write() -> Result<()> {
    Ok(())
  }
}

impl Default for HgWebConfig {
  fn default() -> Self {
    HgWebConfig::new()
  }
}

#[test]
fn hgweb_test() {
  let mut web_conf = HgWebConfig::default();

  web_conf
    .paths
    .insert(PathBuf::from("foo"), PathBuf::from("bar"));

  let wc2 = web_conf.paths.try_insert(
    PathBuf::from("contrib/lib/rust/tempdir"),
    PathBuf::from("contrib/lib/rust/tempdir"),
  );

  assert!(wc2.is_ok());
}
