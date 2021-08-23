//! cfg::config::repo
//!
//! Repo configuration primitives

use std::{
  collections::HashMap,
  fmt,
  fs::File,
  io::{LineWriter, Read, Write},
  net::SocketAddr,
  path::{Path, PathBuf},
};

use crate::Result;
pub use git2::Repository as GitRepository;
pub use hg_parser::MercurialRepository;
use logger::log::{info, trace};
use serde::{Deserialize, Serialize};

/// Generic repo configuration type
///
/// Wraps Mercurial and/or Git repos
#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct RepoConfig {
  vcs: String,
  origin: String,
  path: PathBuf,
}

impl RepoConfig {
  /// Create a new RepoConfig
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

/// The type of Repo (Mercurial or Git)
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

/// Subrepo type
///
/// Note that Mercurial subrepos are a 'feature of last resort'
/// according to official docs. They are needed in very niche
/// scenarios and shouldn't be used most of the time.
pub struct SubRepo {
  pub vcs: String,
  pub origin: String,
  pub path: String,
}

/// Mercurial '.hgsub' file handle, which is just a list of PATH=SOURCE pairs.
pub struct HgSubFile {
  pub path: PathBuf,
  pub subrepos: Vec<SubRepo>,
}

impl HgSubFile {
  /// Create a new '.hgsub' file in current directory
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

  /// TODO [2021-08-13 Fri]
  /// insert a subrepo into this HgSubFile. does not clone the source
  /// or ensure that path exists. Takes an optional argument of 'hg'
  /// or 'git' to indicate the subrepo-type. None represents a
  /// local-only repo.
  pub fn insert(&self, _name: &str, _vcs: Option<&str>) -> Result<()> {
    Ok(())
  }

  /// Sort the full contents of a .hgsub file alphabetically.
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

/// Hgweb configuration type
///
/// Based on the configuration file for 'hgweb' scripts.
///
/// We don't remember the file path in this case because all
/// HgwebConfig values are relative to $PWD.
#[derive(Serialize, Deserialize, Debug)]
pub struct HgwebConfig {
  pub name: String,
  pub contact: String,
  pub description: String,
  pub extensions: Vec<String>,
  pub sock: SocketAddr,
  pub paths: HashMap<PathBuf, PathBuf>,
}

impl HgwebConfig {
  /// Create a new HgwebConfig
  pub fn new() -> Self {
    HgwebConfig {
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
  /// Return HgwebConfig from hgweb configuration file
  pub fn from_path(_path: &Path) -> Self {
    HgwebConfig::new()
  }

  /// Write a HgwebConfig to file
  pub fn write() -> Result<()> {
    Ok(())
  }
}

impl Default for HgwebConfig {
  fn default() -> Self {
    HgwebConfig::new()
  }
}

/// ensure ability to insert new entries to HgwebConfig
#[test]
fn hgweb_test() {
  let mut web_conf = HgwebConfig::default();

  web_conf
    .paths
    .insert(PathBuf::from("foo"), PathBuf::from("bar"));

  let wc2 = web_conf.paths.try_insert(
    PathBuf::from("contrib/lib/rust/tempdir"),
    PathBuf::from("contrib/lib/rust/tempdir"),
  );

  assert!(wc2.is_ok());
}
