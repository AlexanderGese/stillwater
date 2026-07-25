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

