use std::{fs::File, path::Path};

use cfg::ShedConfig;
use net::client::{
  ipapi::get_ip,
  nws::{get_forecast, get_point},
  Client, APP_USER_AGENT,
};
pub use sys::flate::pack;
use sys::logger::log::info;
use weather::Point;

pub mod pass;
pub mod shell;

pub fn load_config(path: &str) {
  let cfg: ShedConfig = ShedConfig::load(path).unwrap();
  println!("ShedConfig: {:?}", cfg);
}

/// Write the given Configuration to a config.ron file.
pub fn write_config(config: ShedConfig, output: &Path) {
  config.write(output).expect("should write config to output");
}

pub fn usb_devices() {
  for device in rusb::devices().unwrap().iter() {
    let device_desc = device.device_descriptor().unwrap();
    println!(
      "Bus {:03} Device {:03} ID {:04x}:{:04x}",
      device.bus_number(),
      device.address(),
      device_desc.vendor_id(),
      device_desc.product_id()
    );
  }
}

pub async fn weather_report() {
  let client = Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .unwrap();
  let file = dirs_next::config_dir()
    .expect("bad XDG_CONFIG!")
    .join("user.ron");
  println!("home is: {:?}", &file);
  let user_point = File::open(file).expect("user.ron is no good!");
  let point: Point = match ron::de::from_reader(user_point) {
    Ok(x) => {
      println!("{:#?}", &x);
      x
    }
    Err(e) => {
      println!("Failed to load config: {}", e);

      std::process::exit(1);
    }
  };

  let res = get_point(&point, &client)
    .await
    .expect("could not get point!");
  let resf = get_forecast(&res, &client).await.unwrap();
  for i in resf.properties.periods.iter() {
    println!("{:#?}, {:#?}", &i.name, &i.detailed_forecast);
  }
}

pub fn print_org(path: &str) {
  org::print_ron(path).expect("could not print org-file!");
}

pub async fn my_ip() {
  get_ip().await.expect("should return IP");
}
