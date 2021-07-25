use weather::{Forecast, Point, PointInfo};

use super::Client;
use crate::Result;

pub async fn get_point(pnt: &Point, client: &Client) -> Result<PointInfo> {
  let mut url: String = String::from("http://api.weather.gov/");
  for i in &["points/", &pnt.lat.to_string(), ",", &pnt.lng.to_string()] {
    url.push_str(i);
  }
  let response = client.get(&url).send().await?;
  let body = response.text().await?;
  let res: PointInfo = serde_json::from_str(&body)?;
  Ok(res)
}

pub async fn get_forecast(pnt: &PointInfo, client: &Client) -> Result<Forecast> {
  let response = client.get(&pnt.properties.forecast).send().await?;
  let body = response.text().await?;
  let res: Forecast = serde_json::from_str(&body)?;
  Ok(res)
}

pub async fn get_forecast_hourly(pnt: &PointInfo, client: &Client) -> Result<Forecast> {
  let response = client.get(&pnt.properties.forecast_hourly).send().await?;
  let body = response.text().await?;
  let res: Forecast = serde_json::from_str(&body)?;
  Ok(res)
}
