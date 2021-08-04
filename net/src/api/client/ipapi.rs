use super::{Client, Error};
use logger::log::trace;
use std::net::IpAddr;
pub async fn my_ip_verbose() -> Result<(), Error> {
  let echo_json: serde_json::Value = Client::new()
    .get("https://ipwhois.app/json/")
    .send()
    .await?
    .json()
    .await?;

  trace!("{:#?}", echo_json);
  Ok(())
}

pub async fn my_ip() -> Result<IpAddr, Error> {
  let res = Client::new().get("https://ipinfo.io/ip").send().await?.text().await?;
  trace!("{:#?}", res);
  Ok(res.parse().unwrap())
}
