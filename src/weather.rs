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
        // Fall: foggy, rainy, cloudy, with light frost creeping in; no snow.
        Season::Fall => {
            if roll < 25 {
                Weather::Fog
            } else if roll < 50 {
                Weather::Rain
            } else if roll < 75 {
                Weather::Cloudy
            } else if roll < 90 {
                Weather::Sunny
            } else if roll < 95 {
                Weather::Storm
            } else {
                Weather::Frost
            }
        }
        // Winter: snow, frost, cloudy; no rain/storm/fog.
        Season::Winter => {
            if roll < 35 {
                Weather::Snow
            } else if roll < 65 {
                Weather::Frost
            } else if roll < 90 {
                Weather::Cloudy
            } else {
                Weather::Sunny
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_is_deterministic_for_same_seed() {
        let mut a = Rng::new(1234);
        let mut b = Rng::new(1234);
        for _ in 0..200 {
            assert_eq!(roll(Season::Spring, &mut a), roll(Season::Spring, &mut b));
        }
    }

    #[test]
    fn roll_is_deterministic_across_all_seasons() {
        for season in Season::all() {
            let mut a = Rng::new(999);
            let mut b = Rng::new(999);
            let seq_a: Vec<Weather> = (0..50).map(|_| roll(season, &mut a)).collect();
            let seq_b: Vec<Weather> = (0..50).map(|_| roll(season, &mut b)).collect();
            assert_eq!(seq_a, seq_b);
        }
    }

    #[test]
    fn winter_never_rains_or_storms_or_fogs() {
        let mut rng = Rng::new(7);
        for _ in 0..2000 {
            let w = roll(Season::Winter, &mut rng);
            assert_ne!(w, Weather::Rain);
            assert_ne!(w, Weather::Storm);
            assert_ne!(w, Weather::Fog);
        }
    }

    #[test]
    fn only_winter_produces_snow() {
        let mut rng = Rng::new(11);
        for season in [Season::Spring, Season::Summer, Season::Fall] {
            for _ in 0..2000 {
                assert_ne!(roll(season, &mut rng), Weather::Snow);
            }
        }
        // Winter must actually be able to roll snow.
        let mut winter_rng = Rng::new(13);
        let saw_snow = (0..2000).any(|_| roll(Season::Winter, &mut winter_rng) == Weather::Snow);
        assert!(saw_snow);
    }

    #[test]
    fn summer_never_snows_or_frosts() {
        let mut rng = Rng::new(21);
        for _ in 0..2000 {
            let w = roll(Season::Summer, &mut rng);
            assert_ne!(w, Weather::Snow);
            assert_ne!(w, Weather::Frost);
        }
    }

    #[test]
    fn is_wet_only_for_rain_and_storm() {
        for w in [
            Weather::Sunny,
            Weather::Cloudy,
            Weather::Rain,
            Weather::Storm,
            Weather::Fog,
            Weather::Frost,
            Weather::Snow,
        ] {
            let expected = matches!(w, Weather::Rain | Weather::Storm);
            assert_eq!(w.is_wet(), expected);
        }
    }

