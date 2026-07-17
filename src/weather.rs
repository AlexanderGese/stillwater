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

    /// A single-char HUD glyph for the weather.
    pub fn glyph(self) -> char {
        match self {
            Weather::Sunny => '☀',
            Weather::Cloudy => '☁',
            Weather::Rain => '☔',
            Weather::Storm => '⚡',
            Weather::Fog => '≈',
            Weather::Frost => '✻',
            Weather::Snow => '❄',
        }
    }

    /// Wet weather auto-waters crops and dampens the ground (rain/storm).
    pub fn is_wet(self) -> bool {
        matches!(self, Weather::Rain | Weather::Storm)
    }

    /// Percent modifier applied to fishing bite chance (e.g. Rain => +25).
    /// Storm surfaces big fish and stirs the water; fog/rain boost bites by
    /// masking the angler; frost/snow chill the water and reduce bites.
    pub fn bite_bonus(self) -> i32 {
        match self {
            Weather::Sunny => 0,
            Weather::Cloudy => 5,
            Weather::Rain => 25,
            Weather::Storm => 15,
            Weather::Fog => 15,
            Weather::Frost => -15,
            Weather::Snow => -20,
        }
    }
}

