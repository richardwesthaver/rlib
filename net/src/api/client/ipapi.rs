use super::{Client, Error};

pub async fn get_ip() -> Result<(), Error> {
  let echo_json: serde_json::Value = Client::new()
    .get("https://ipwhois.app/json/")
    .send()
    .await?
    .json()
    .await?;

  println!("{:#?}", echo_json);
  Ok(())
}
