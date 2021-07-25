//! TODO: replace structs with plain functions
use super::{Arg, Cmd, SubCommand};

pub fn pack_cmd() -> Cmd {
  SubCommand::with_name("pack")
    .about("Build packages.")
    .args(&[
      Arg::with_name("output")
        .short("o")
        .help("Specify an output location, overriding config and default values.")
        .takes_value(true),
      Arg::with_name("input")
        .short("i")
        .help("Specify an input.")
        .takes_value(true),
    ])
}

pub fn write_cfg_cmd() -> Cmd {
  SubCommand::with_name("write")
    .alias("w")
    .about("Write current configuration to file.")
    .arg(
      Arg::with_name("output")
        .short("o")
        .takes_value(true)
        .value_name("RON_FILE")
        .help("Specify an output file location."),
    )
}

pub fn load_cfg_cmd() -> Cmd {
  SubCommand::with_name("load")
    .about("Load configuration from file.")
    .arg(
      Arg::with_name("input")
        .short("i")
        .takes_value(true)
        .value_name("RON_FILE")
        .help("Specify an input file location."),
    )
}

pub fn make_cmd() -> Cmd {
  SubCommand::with_name("make")
    .about("MakeFile targets.")
    .arg(
      Arg::with_name("target")
        .short("t")
        .help("Specify a MakeFile target.")
        .takes_value(true)
        .value_name("MAKE_TARGET"),
    )
}

pub fn usb_cmd() -> Cmd {
  SubCommand::with_name("usb").about("List USB Devices.").arg(
    Arg::with_name("list")
      .short("l")
      .help("list usb devices [default]"),
  )
}

pub fn weather_cmd() -> Cmd {
  SubCommand::with_name("weather")
    .about("Print a Weather Forecast.")
    .arg(
      Arg::with_name("location")
        .short("l")
        .takes_value(true)
        .value_name("CITY")
        .help("location to query."),
    )
    .arg(
      Arg::with_name("point")
        .short("p")
        .takes_value(true)
        .value_name("(LAT LNG)")
        .help("specify a lat,lng pair to query."),
    )
}
