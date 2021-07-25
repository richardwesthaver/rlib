//    load_config, make, my_ip, pack, parse_org, usb_devices,
//    weather_report, write_config,
pub mod app;
pub mod ctl;
mod err;
use crate::err::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
