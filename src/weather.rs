//! Daily weather. CONTRACT STUB — an implementer agent fills the method bodies,
//! the seasonal `roll` weights, and tests. Keep the public names/signatures.

use crate::rng::Rng;
use crate::season::Season;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rain,
    Storm,
    Fog,
    Frost,
    Snow,
}

impl Weather {
    pub fn name(self) -> &'static str {
        match self {
            Weather::Sunny => "sunny",
            Weather::Cloudy => "cloudy",
            Weather::Rain => "rain",
            Weather::Storm => "storm",
            Weather::Fog => "fog",
            Weather::Frost => "frost",
            Weather::Snow => "snow",
        }
    }

