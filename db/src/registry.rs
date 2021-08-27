//! registry module
use crate::RocksDB;

/// Registry handle
pub struct Registry {
  pub db: RocksDB,
}

impl Registry {
  pub fn new() -> Self {
    let db = RocksDB::new();
    Registry { db }
  }
}
