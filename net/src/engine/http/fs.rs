use axum::response::{Html, IntoResponse};
use reqwest::StatusCode;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;

pub async fn handler() -> Html<&'static str> {
  Html("<h1>hey there neighbour</h1>")
}

pub async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}

pub struct FileServer {
  pub socket: SocketAddr,
  pub path: PathBuf,
  pub registry: HashMap<String, String>,
}
