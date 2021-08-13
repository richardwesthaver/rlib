use std::net::UdpSocket;
use weather::Point;
use crate::client::*;

#[test]
fn udp_start() {
  let socket = UdpSocket::bind("127.0.0.1:0").expect("UdpSocket::bind failed");
  assert!(socket.broadcast().is_ok());
}

#[ctx::test]
async fn ip_lookup() {
  assert!(ipapi::my_ip().await.is_ok());
  assert!(ipapi::my_ip_verbose().await.is_ok());
}

#[ctx::test]
async fn nwsapi() {
  let pnt = Point::new(41.320361, -72.063304);
  let client = Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .unwrap();
  let res = nws::get_point(&pnt, &client).await.unwrap();
  let resf = nws::get_forecast(&res, &client).await;
  assert!(resf.is_ok());
}
