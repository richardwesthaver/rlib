use std::net::UdpSocket;
use weather::Point;
use crate::client::*;

#[test]
fn udp_start() {
  let socket = UdpSocket::bind("127.0.0.1:0").expect("UdpSocket::bind failed");
  assert!(socket.broadcast().is_ok());
}
