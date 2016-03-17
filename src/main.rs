#[macro_use]
extern crate tinyosc;
use std::borrow::Cow;

mod weather_provider;

fn to_string<'a>(x : &'a weather_provider::WeatherCondition) -> Cow<'a, str> {
  Cow::Owned(format!("{:?}", x))
}

fn to_osc<'a>(address : &'a str, x: weather_provider::WeatherRecord<'a>) -> tinyosc::Message<'a> {
  tinyosc::Message {
    path: address,
    arguments: vec![
       tinyosc::Argument::s(&x.location),
       tinyosc::Argument::i(x.temperature),
    ]
  }
}

fn main() {
    let weather = weather_provider::WeatherRecord::new();
    let msg = to_osc("/slots/weather/bellevue",
      weather_provider::query(&weather, "Bellevue, WA")
    );
    println!("{:?}", msg.serialize());
}
