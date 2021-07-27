//! db::registry
use crate::RocksDB;

pub struct Registry {
  db: RocksDB,
}

impl Registry {
  pub fn new() -> Self {
    let db = RocksDB::new();
    Registry { db }
  }
}
