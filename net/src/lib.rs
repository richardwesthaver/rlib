mod api;
mod err;
mod transport;
mod udp;
pub use crate::{api::client, err::Error};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
