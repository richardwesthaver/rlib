use std::collections::HashMap;

pub use flate::pack;
use logger::log::{info, debug};
use obj::Org;
use obj::Objective;
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

pub fn print_org(path: &str) {
  let doc = Org::from_file(path).unwrap();
  doc.to_ron_string().unwrap();
}
