mod api;
mod err;
pub use crate::{
  api::{
    client::{self, Client},
    server::Server,
  },
  err::Error,
};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
