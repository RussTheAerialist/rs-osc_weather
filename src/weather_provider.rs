use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
pub enum WeatherCondition {
  Sunny
}

pub struct WeatherRecord {
  pub location: &'static str,
  pub temperature: i32,
  pub condition: WeatherCondition
}

impl Display for WeatherRecord {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{} {:?} ({}F)", self.location, self.condition, self.temperature)
  }
}

pub fn query(location: &str) -> WeatherRecord {
  WeatherRecord {
    location: location,
    temperature: 65,
    condition: WeatherCondition::Sunny
  }
}
