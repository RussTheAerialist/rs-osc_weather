use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
pub enum WeatherCondition {
  Unknown,
  Sunny
}

pub struct WeatherRecord<'a> {
  pub location: &'a str,
  pub temperature: i32,
  pub condition: WeatherCondition
}

impl<'a> WeatherRecord<'a> {
  pub fn new() -> WeatherRecord<'a> {
    let loc : &'static str = "Unknown";
    WeatherRecord {
      location: loc,
      temperature: -1,
      condition: WeatherCondition::Unknown
    }
  }
}

impl<'a> Display for WeatherRecord<'a> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{} {:?} ({}F)", self.location, self.condition, self.temperature)
  }
}

pub fn query<'a>(record: &'a WeatherRecord, location: &'a str) -> WeatherRecord<'a> {
  WeatherRecord { location: location, temperature: 65, condition: WeatherCondition::Sunny , .. *record }
}

