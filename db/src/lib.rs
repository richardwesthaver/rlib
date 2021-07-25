use std::path::PathBuf;
use std::sync::Arc;

pub use rocksdb::{ColumnFamilyDescriptor, Options, DB};

mod err;
#[cfg(test)]
mod tests;

pub use crate::err::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct RocksDB {
  path: PathBuf,
  db: Arc<DB>,
}

impl RocksDB {
  pub fn new() -> Self {
    let path = PathBuf::from("./default_db");
    let db = DB::open_default(&path).unwrap();
    let db = Arc::new(db);
    RocksDB { path: path, db: db }
  }
}
