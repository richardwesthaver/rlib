use crate::Result;
pub use flate::pack;
use logger::log::debug;
use std::collections::HashMap;

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
    println!(
      "Bus {:03} Device {:03} ID {:04x}:{:04x}",
      device.bus_number(),
      device.address(),
      device_desc.vendor_id(),
      device_desc.product_id()
    );

    match usbmap.get(&(device_desc.vendor_id(), device_desc.product_id())) {
      Some(m) => println!(
        "found device {} at bus:{:03} dev:{:03}",
        m,
        device.bus_number(),
        device.address()
      ),
      None => debug!(
        "bus:{:03} dev:{:03} unknown",
        device.bus_number(),
        device.address()
      ),
    }
  }
  Ok(())
}
