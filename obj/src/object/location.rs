use chrono::{DateTime, Utc};

use super::{Deserialize, Objective, Serialize};

/// A City object descriptor. Serves as an anchor for many properties
/// in location-based data.
#[derive(Serialize, Deserialize, Debug)]
pub struct City {
  point: Point,
  area: Vec<Point>,
  name: String,
  region: String,
  county: String,
  state: String,
  timezone: DateTime<Utc>,
}

impl Objective for City {}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Point {
  x: f32,
  y: f32,
}

impl Point {
  // create a new 'Point' from (f32, f32)
  pub fn new(x: f32, y: f32) -> Self {
    Point { x, y }
  }
}

impl From<City> for Point {
  fn from(city: City) -> Self {
    city.point
  }
}

impl Objective for Point {}
