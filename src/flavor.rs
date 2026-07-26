//! Cozy flavor text. CONTRACT STUB — an implementer agent fills the phrase
//! tables and picks one deterministically via the rng. Keep the signatures.

use crate::rng::Rng;
use crate::weather::Weather;

/// A gentle morning greeting line. AGENT: fill a small table, pick via rng.
pub fn greeting(rng: &mut Rng) -> &'static str {
    const LINES: [&str; 8] = [
        "The lake mist is lifting.",
        "Birdsong drifts over the water.",
        "A new day at Stillwater.",
        "Dew clings to the grass by the dock.",
        "Sunlight climbs slowly over the ridge.",
        "The water is glassy and still.",
        "Somewhere a rooster is greeting the morning.",
        "Smoke curls gently from a chimney across the lake.",
    ];
    LINES[rng.below(LINES.len() as u32) as usize]
}

/// A short line describing the day's weather/mood. AGENT: vary by weather.
pub fn weather_line(w: Weather, rng: &mut Rng) -> &'static str {
    match w {
        Weather::Sunny => {
            const LINES: [&str; 4] = [
                "The sun feels warm on your shoulders.",
                "A bright, clear day for fishing.",
                "Light sparkles across the water.",
                "Not a cloud in the sky.",
            ];
            LINES[rng.below(LINES.len() as u32) as usize]
        }
        Weather::Cloudy => {
            const LINES: [&str; 4] = [
                "Soft grey clouds drift overhead.",
                "The light is gentle and even today.",
                "A cool breeze moves under a hazy sky.",
                "The clouds hang low and quiet.",
            ];
            LINES[rng.below(LINES.len() as u32) as usize]
        }
        Weather::Rain => {
            const LINES: [&str; 4] = [
                "Rain patters softly on the water.",
                "A gentle rain settles the dust.",
                "The lake ripples under a steady drizzle.",
                "Raindrops ring out on your hat brim.",
            ];
            LINES[rng.below(LINES.len() as u32) as usize]
        }
        Weather::Storm => {
            const LINES: [&str; 4] = [
                "Thunder rumbles somewhere far off.",
                "The wind whips the reeds along the shore.",
                "Dark clouds gather over the ridge.",
                "The lake churns under a restless sky.",
            ];
            LINES[rng.below(LINES.len() as u32) as usize]
        }
        Weather::Fog => {
            const LINES: [&str; 4] = [
                "A soft fog blankets the water.",
                "The far shore is lost in mist.",
                "Everything feels hushed and close.",
                "Fog curls low over the still lake.",
            ];
            LINES[rng.below(LINES.len() as u32) as usize]
        }
        Weather::Frost => {
            const LINES: [&str; 4] = [
                "Frost glitters on the dock boards.",
                "Your breath fogs in the crisp air.",
                "The grass crunches, stiff with frost.",
                "A cold, clear morning settles in.",
            ];
            LINES[rng.below(LINES.len() as u32) as usize]
        }
        Weather::Snow => {
            const LINES: [&str; 4] = [
                "Snow drifts down in slow, soft flakes.",
                "A quiet blanket of snow covers the shore.",
                "The world feels muffled and calm.",
                "Snowflakes settle gently on the water's edge.",
            ];
            LINES[rng.below(LINES.len() as u32) as usize]
        }
    }
}

/// A little line when you land a fish. AGENT: fill a small table.
pub fn catch_line(rng: &mut Rng) -> &'static str {
    const LINES: [&str; 8] = [
        "Nice catch.",
        "A good one!",
        "It put up a fight.",
        "Reeled in nice and steady.",
        "That's a keeper.",
        "Right on the line.",
        "A fine catch for the basket.",
        "Well earned, that one.",
    ];
    LINES[rng.below(LINES.len() as u32) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn greeting_is_non_empty() {
        let mut rng = Rng::new(7);
        assert!(!greeting(&mut rng).is_empty());
    }

    #[test]
    fn catch_line_is_non_empty() {
        let mut rng = Rng::new(7);
        assert!(!catch_line(&mut rng).is_empty());
    }

    #[test]
    fn weather_line_non_empty_for_all_variants() {
        let variants = [
            Weather::Sunny,
            Weather::Cloudy,
            Weather::Rain,
            Weather::Storm,
            Weather::Fog,
            Weather::Frost,
            Weather::Snow,
        ];
        for w in variants {
            let mut rng = Rng::new(11);
            assert!(!weather_line(w, &mut rng).is_empty());
        }
    }

    #[test]
    fn greeting_deterministic_for_fixed_seed() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..20 {
            assert_eq!(greeting(&mut a), greeting(&mut b));
        }
    }

    #[test]
    fn catch_line_deterministic_for_fixed_seed() {
        let mut a = Rng::new(99);
        let mut b = Rng::new(99);
        for _ in 0..20 {
            assert_eq!(catch_line(&mut a), catch_line(&mut b));
        }
    }

    #[test]
    fn weather_line_deterministic_for_fixed_seed() {
        let mut a = Rng::new(123);
        let mut b = Rng::new(123);
        for _ in 0..20 {
            assert_eq!(
                weather_line(Weather::Storm, &mut a),
                weather_line(Weather::Storm, &mut b)
            );
        }
    }

    #[test]
    fn greeting_varies_across_rng_states() {
        let mut rng = Rng::new(1);
        let mut seen = HashSet::new();
        for _ in 0..50 {
            seen.insert(greeting(&mut rng));
        }
        assert!(seen.len() > 1, "expected multiple distinct greetings");
    }

