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
