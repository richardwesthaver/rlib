mod err;
mod object;
mod paths;
pub use crate::{
  err::Error,
  object::{Doc, Meta, Note, ObjectId, Objective, Org, Properties},
};

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Error>;
