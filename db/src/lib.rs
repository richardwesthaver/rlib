use std::path::PathBuf;
use std::sync::Arc;

pub(crate) use rocksdb::{ColumnFamilyDescriptor, Options, DB};

mod err;
mod registry;
pub use crate::{err::Error, registry::Registry};

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Error>;

pub struct RocksDB {
  path: PathBuf,
  db: Arc<DB>,
}

impl RocksDB {
  pub fn new() -> Self {
    let path = PathBuf::from(".rdb");
    let db = DB::open_default(&path).unwrap();
    let db = Arc::new(db);
    RocksDB { path: path, db: db }
  }
}
