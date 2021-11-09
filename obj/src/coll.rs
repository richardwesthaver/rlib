//! # coll
use crate::Objective;

use hash::Id;
use std::collections::BTreeMap;

/// Collection container trait for single-typed sets
pub struct Coll<T: Objective>(Vec<T>);

pub struct Collection<T: Objective>(BTreeMap<Id, T>);
