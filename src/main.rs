#[macro_use]
extern crate tinyosc;
use std::net;

mod weather_provider;

fn to_osc<'a>(address : &'a str, x: weather_provider::WeatherRecord<'a>) -> tinyosc::Message<'a> {
  tinyosc::Message {
    path: address,
    arguments: vec![
       tinyosc::Argument::s(&x.location),
       tinyosc::Argument::i(x.temperature),
    ]
  }
}

fn get_weather() -> Result<bool, std::io::Error> {
    let ip = net::Ipv4Addr::new(127,0,0,1);
    let send_addr = net::SocketAddrV4::new(ip, 9998);
    let address = net::SocketAddrV4::new(ip, 9999);
    let weather = weather_provider::WeatherRecord::new();
    let msg = to_osc("/slots/weather/bellevue",
      weather_provider::query(&weather, "Bellevue, WA")
    );
    let packet = try!(msg.serialize());
    let socket = try!(net::UdpSocket::bind(net::SocketAddr::V4(send_addr)));
    try!(socket.send_to(&packet, net::SocketAddr::V4(address)));
    Ok(true)
}

fn main() {
    match get_weather() {
        Ok(_) => println!("Finished"),
        Err(why) => panic!("Error {}", why)
    }
}
