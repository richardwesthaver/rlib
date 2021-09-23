mod client;
mod resolver;

use std::{collections::HashMap, net::IpAddr};

pub type IpTable = HashMap<IpAddr, String>;
