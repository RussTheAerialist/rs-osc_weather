#[macro_use]
extern crate tinyosc;
use std::fmt;

mod weather_provider;

fn to_string(x : weather_provider::WeatherCondition) -> &'static str {
  &(format!("{:?}", x))[..]
}

fn to_osc(address : &str, x : weather_provider::WeatherRecord) -> tinyosc::Message {
  tinyosc::Message {
    path: address,
    arguments: vec![
       tinyosc::Argument::s(&String::from(x.location)[..]),
       tinyosc::Argument::i(x.temperature),
       tinyosc::Argument::s(to_string(x.condition))
    ]
  }
}

fn main() {
    let msg = to_osc("/slots/weather/bellevue",
      weather_provider::query("Bellevue, WA")
    );
    println!("{:?}", msg.serialize());
}
