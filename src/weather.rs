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

/// Roll tomorrow's weather from this season's weighted distribution.
/// Deterministic given the rng: draws a single `rng.below(100)` value and
/// walks cumulative weight thresholds per season.
pub fn roll(season: Season, rng: &mut Rng) -> Weather {
    let roll = rng.below(100);
    match season {
        // Spring: rainy and cloudy, with a little fog/storm; no frost/snow.
        Season::Spring => {
            if roll < 35 {
                Weather::Rain
            } else if roll < 65 {
                Weather::Cloudy
            } else if roll < 90 {
                Weather::Sunny
            } else if roll < 95 {
                Weather::Fog
            } else {
                Weather::Storm
            }
        }
        // Summer: mostly sunny with occasional storms; no frost/snow/fog-heavy.
        Season::Summer => {
            if roll < 60 {
                Weather::Sunny
            } else if roll < 75 {
                Weather::Cloudy
            } else if roll < 90 {
                Weather::Storm
            } else if roll < 98 {
                Weather::Rain
            } else {
                Weather::Fog
            }
        }
