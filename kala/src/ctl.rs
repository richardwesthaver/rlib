use std::{fs::File, path::PathBuf, str::FromStr};

pub use flate::pack;
use logger::log::{error, trace};
use net::client::{
  ipapi::my_ip,
  nws::{get_forecast, get_point},
  Client, APP_USER_AGENT,
};
use weather::Point;

pub fn usb_devices() {
  println!("---------------------------------------");
  println!("--++-- USB DEVICES --++--");
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
  let mut file =
    PathBuf::from_str(option_env!("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME not found"))
      .unwrap()
      .join("user.ron");
  trace!("user/cfg :=: {:?}", &file);
  let user_point = File::open(file).expect("user.ron is no good!");
  let point: Point = match ron::de::from_reader(user_point) {
    Ok(x) => x,
    Err(e) => {
      error!("Failed to load config: {}", e);

      std::process::exit(1);
    }
  };

  let res = get_point(&point, &client)
    .await
    .expect("could not get point!");
  let resf = get_forecast(&res, &client)
    .await
    .expect("could not get forecast");

  println!("---------------------------------------");
  println!("--++-- WEATHER FORECAST --++--");
  for i in resf.properties.periods.iter() {
    println!("------------");
    println!("{:#?} -:- {:#?}", &i.name, &i.detailed_forecast);
  }
}

pub fn print_org(path: &str) {
  org::print_ron(path).expect("could not print org-file!");
}

pub async fn get_ip() {
  let ip = my_ip().await.expect("should return IP");
  println!("---------------------------------------");
  println!("--++-- PUBLIC_IP === {:#?} --++--", ip);
}
