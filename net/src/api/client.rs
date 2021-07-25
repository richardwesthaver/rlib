use std::env;

pub use reqwest::{Client, Error, StatusCode};

pub mod ipapi;
pub mod nws;

/// User-Agent HTTP Header value
pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
