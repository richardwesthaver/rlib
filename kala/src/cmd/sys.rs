//! kala::cmd::sys modules
//!
//! system info commands, for the self-conscious.
use crate::Result;
use ctx::tokio::process::Command;
pub use flate::pack;
use logger::log::debug;
use std::collections::HashMap;

/// OS-specific browser command. supports Win/Mac/Linux
pub async fn open_browser(url: &str) {
  if cfg!(target_os = "windows") {
    // https://stackoverflow.com/a/49115945
    Command::new("rundll32.exe")
      .args(&["url.dll,FileProtocolHandler", url])
      .status()
      .await
      .expect("failed to open file");
  } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
    // https://dwheeler.com/essays/open-files-urls.html
    #[cfg(target_os = "macos")]
    let cmd = "open";
    #[cfg(target_os = "linux")]
    let cmd = "xdg-open";

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      Command::new(cmd)
        .arg(url)
        .status()
        .await
        .expect("failed to open file");
    }
  } else {
    unimplemented!() //Ignore others
  }
}

/// Print information about the current host using `whoami` crate
pub fn describe_host() {
  println!("User's Name:            {}", whoami::realname());
  println!("User's Username:        {}", whoami::username());
  println!(
    "User's Language:        {:?}",
    whoami::lang().collect::<Vec<String>>()
  );
  println!("Device's Pretty Name:   {}", whoami::devicename());
  println!("Device's Hostname:      {}", whoami::hostname());
  println!("Device's Platform:      {}", whoami::platform());
  println!("Device's OS Distro:     {}", whoami::distro());
  println!("Device's Desktop Env:   {}", whoami::desktop_env());
}

/// Print info about the USB devices found on host. If Some(`devices`)
/// mapping is provided, look up devices and return human-readable
/// info.
pub fn usb_devices(devices: Option<HashMap<(u16, u16), String>>) -> Result<()> {
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

    if let Some(ref usbmap) = devices {
      match usbmap.get(&(device_desc.vendor_id(), device_desc.product_id())) {
        Some(m) => println!(
          "found device {} at bus:{:03} dev:{:03}",
          m,
          device.bus_number(),
          device.address()
        ),
        None => println!(
          "bus:{:03} dev:{:03} unknown",
          device.bus_number(),
          device.address()
        ),
      }
    };
  }
  println!("--++--");
  Ok(())
}
