use std::collections::HashMap;
use std::{fs::File, path::PathBuf, str::FromStr};

pub use flate::pack;
use logger::log::{error, info, debug};
use net::client::{
  ipapi::my_ip,
  nws::{get_forecast, get_point},
  Client, APP_USER_AGENT,
};
use obj::Org;
use obj::Objective;
use weather::Point;

use crate::Result;

pub fn usb_devices(usbmap: HashMap<(u16, u16), String>) -> Result<()> {
  // basic local info, useful for debugging
  debug!("has capability? {}", rusb::has_capability());
  debug!("has hotplug? {}", rusb::has_hotplug());
  debug!("has HID access? {}", rusb::has_hid_access());
  debug!(
    "supports detach kernel driver? {}",
    rusb::supports_detach_kernel_driver()
  );

  for device in rusb::devices().unwrap().iter() {
    let device_desc = device.device_descriptor().unwrap();
    info!(
      "Bus {:03} Device {:03} ID {:04x}:{:04x}",
      device.bus_number(),
      device.address(),
      device_desc.vendor_id(),
      device_desc.product_id()
    );

    match usbmap.get(&(device_desc.vendor_id(), device_desc.product_id())) {
      Some(m) => info!(
        "found device {} at bus:{:03} dev:{:03}",
        m,
        device.bus_number(),
        device.address()
      ),
      None => info!(
        "bus:{:03} dev:{:03} unknown",
        device.bus_number(),
        device.address()
      ),
    }
  }
  Ok(())
}

pub async fn weather_report() {
  let client = Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .unwrap();

  let file = PathBuf::from_str(option_env!("XDG_CONFIG_HOME").unwrap()).unwrap().join("user.ron");

  debug!("user/cfg :=: {:?}", &file);
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

  for i in resf.properties.periods.iter() {
    println!("------------");
    println!("{:#?} -:- {:#?}", &i.name, &i.detailed_forecast);
  }
}

pub fn print_org(path: &str) {
  let doc = Org::from_file(path).unwrap();
  doc.to_ron_string().unwrap();
}

pub async fn get_ip() {
  let ip = my_ip().await.expect("should return IP");
  println!("--++-- PUBLIC_IP === {:#?} --++--", ip);
}
