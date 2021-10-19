//! db modules
//!
//! This library provides types and builder functions for working with
//! databases. Currently the only backend supported is RocksDB.
mod err;
pub mod cache;
mod meter;
pub mod registry;
pub use err::{Error, Result};

pub use rocksdb::{ColumnFamilyDescriptor, Options, DB};

use std::path::PathBuf;
use std::sync::Arc;

#[cfg(test)]
mod tests;

/// RocksDB handle
pub struct RocksDB {
  pub path: PathBuf,
  pub db: Arc<DB>,
}

impl RocksDB {
  pub fn new() -> Self {
    let path = PathBuf::from(".rdb");
    let db = DB::open_default(&path).unwrap();
    let db = Arc::new(db);
    RocksDB { path: path, db: db }
  }
}
