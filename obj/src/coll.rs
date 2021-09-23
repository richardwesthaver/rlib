//! # coll
use crate::Objective;

use std::collections::BTreeMap;
use hash::Id;

/// Collection container trait for single-typed sets
pub struct Coll<T: Objective>(Vec<T>);

pub struct Collection<T: Objective>(BTreeMap<Id, T>);

