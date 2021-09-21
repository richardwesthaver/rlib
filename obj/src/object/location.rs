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

/// Geo-coordinate Point object type
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

  /// Given an additional Point, and assuming Points are on Earth,
  /// returns the distance in kilometers between them using the
  /// Haversine formula
  pub fn earth_distance_from(&self, other: Point) -> f32 {
    let earth_radius_kilometer = 6371.0_f32;
    let lat_rads = self.lat.to_radians();
    let other_lat_rads = other.lat.to_radians();

    let delta_latitude = (self.lat - other.lat).to_radians();
    let delta_longitude = (self.lng - other.lng).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + lat_rads.cos() * other_lat_rads.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between points on the surface of Earth is {:.1} kilometers",
        distance
    );

    distance
  }
}

#[test]
fn london_to_paris() {
  assert_eq!(
    Point::new(48.85341_f32, -2.34880_f32)
      .earth_distance_from(Point::new(51.50853_f32,-0.12574_f32)),
    334.9559_f32,
  );
}
impl From<City> for Point {
  fn from(city: City) -> Self {
    city.point
  }
}

impl Objective for Point {}

