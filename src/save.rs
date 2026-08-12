//! Hand-rolled, versioned save/load (no serde). A save is a small line-based
//! text file: `key value` per line, with a leading `stillwater <version>` tag.

use crate::fish::{self, Catch};
use crate::game::Game;
use crate::journal::Journal;
use crate::season::Season;
use crate::weather::Weather;
use std::fs;
use std::io::Write;

const VERSION: u32 = 1;

fn season_to_u8(s: Season) -> u8 {
    match s {
        Season::Spring => 0,
        Season::Summer => 1,
        Season::Fall => 2,
        Season::Winter => 3,
    }
}
fn u8_to_season(n: u8) -> Season {
    match n {
        1 => Season::Summer,
        2 => Season::Fall,
        3 => Season::Winter,
        _ => Season::Spring,
    }
}
fn weather_to_u8(w: Weather) -> u8 {
    match w {
        Weather::Sunny => 0,
        Weather::Cloudy => 1,
        Weather::Rain => 2,
        Weather::Storm => 3,
        Weather::Fog => 4,
        Weather::Frost => 5,
        Weather::Snow => 6,
    }
}
fn u8_to_weather(n: u8) -> Weather {
    match n {
        1 => Weather::Cloudy,
        2 => Weather::Rain,
        3 => Weather::Storm,
        4 => Weather::Fog,
        5 => Weather::Frost,
