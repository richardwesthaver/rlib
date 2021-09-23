use super::SubRepo;
use crate::Result;
pub use hg_parser::MercurialRepository as HgRepo;
use logger::log::{error, info, trace};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::File,
  io::{LineWriter, Read, Write},
  net::SocketAddr,
  path::PathBuf,
};

/// Mercurial '.hgsub' file handle, which is just a list of PATH=SOURCE pairs.
pub struct HgSubFile {
  pub path: PathBuf,          // path to the .hgsub file
  pub subrepos: Vec<SubRepo>, // Vec containing `SubRepo` handles
}

impl HgSubFile {
  /// Create a new '.hgsub' file handle
  pub fn new() -> Self {
    HgSubFile {
      path: PathBuf::from(".hgsub"),
      subrepos: vec![],
    }
  }
  /// should perform validation to ensure that the path is in a hg repo.
  fn parent_is_hg(&self) -> bool {
    if self.path.parent().unwrap().join(".hg").exists() {
      true
    } else {
      false
    }
  }
  /// insert a subrepo into this HgSubFile. does not clone the source
  /// or ensure that path exists. Takes an optional argument of 'hg'
  /// or 'git' to indicate the subrepo-type. Value can be ommitted to
  pub fn insert(&mut self, path: &str, source: &str, vcs: Option<&str>) -> Result<()> {
    let mut prefix = "";
    // set prefix based on vcs (repo type)
    if let Some(i) = vcs {
      match i {
        "hg" => prefix = "hg",
        "git" => prefix = "git",
        _ => {
          error!("failed to recognize repo type")
        }
      }
    }

    let source = format!("{}:{}", prefix, source);

    let subrepo = SubRepo {
      vcs: vcs.unwrap().to_string(),
      origin: source.to_string(),
      path: path.to_string(),
    };

    self.subrepos.push(subrepo);
    Ok(())
  }

  /// Save subs to `.hgsub` file specified in the `path` field of
  /// this struct. This will overwrite any existing file at that path.
  pub fn save(self) -> Result<()> {
    match self.parent_is_hg() {
      true => {
        let mut file = File::open(self.path).unwrap();
        for i in self.subrepos.iter() {
          write!(file, "{} = {}", i.path, i.origin)?;
        }
      }
      false => {
        error!("Parent is not a Mercurial repo!")
      }
    }
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
      path: PathBuf::from(".hgsub"),
      subrepos: vec![],
    }
  }
}

/// Hgweb configuration type
///
/// Based on the configuration file for 'hgweb' scripts.
///
/// We don't store the file path in a field because all HgwebConfig
/// values are relative to $PWD.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HgwebConfig {
  pub name: String,
  pub contact: String,
  pub description: String,
  pub extensions: Vec<String>,
  pub sock: SocketAddr,
  pub paths: HashMap<PathBuf, PathBuf>,
}

impl Default for HgwebConfig {
  fn default() -> Self {
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
}
