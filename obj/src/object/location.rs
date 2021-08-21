//! obj::location
//!
//! Location object types

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

/// Point object type
///
/// Use in 2D applications.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Point {
  pub lat: f32,
  pub lng: f32,
}

impl Point {
  /// Create a new Point from (f32, f32)
  pub fn new(lat: f32, lng: f32) -> Self {
    Point { lat, lng }
  }
}

impl From<City> for Point {
  fn from(city: City) -> Self {
    city.point
  }
}

impl Objective for Point {}
