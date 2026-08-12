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
        6 => Weather::Snow,
        _ => Weather::Sunny,
    }
}

pub fn serialize(g: &Game) -> String {
    let mut out = String::new();
    out.push_str(&format!("stillwater {}\n", VERSION));
    out.push_str(&format!("year {}\n", g.calendar.year));
    out.push_str(&format!("season {}\n", season_to_u8(g.calendar.season)));
    out.push_str(&format!("day {}\n", g.calendar.day));
    out.push_str(&format!("minutes {}\n", g.clock.minutes));
    out.push_str(&format!("weather {}\n", weather_to_u8(g.weather)));
    out.push_str(&format!("weathernext {}\n", weather_to_u8(g.weather_next)));
    out.push_str(&format!("px {}\n", g.player.pos.x));
    out.push_str(&format!("py {}\n", g.player.pos.y));
    out.push_str(&format!("energy {}\n", g.player.energy));
    out.push_str(&format!("gold {}\n", g.player.gold));
    out.push_str(&format!("rod {}\n", g.player.rod_tier));
    out.push_str(&format!("bait {}\n", g.player.bait_id));
    out.push_str(&format!("area {}\n", g.world.current()));
    let mut fundedline = String::from("funded");
    for i in 0..crate::restore::PROJECTS.len() {
        fundedline.push(' ');
        fundedline.push(if g.world.is_funded(i) { '1' } else { '0' });
    }
    fundedline.push('\n');
    out.push_str(&fundedline);
    let b = |x: bool| if x { 1 } else { 0 };
    out.push_str(&format!(
        "settings {} {} {}\n",
        b(g.settings.hints),
        b(g.settings.color),
        b(g.settings.guide)
    ));
    out.push_str(&format!("legend {}\n", b(g.legend_shown)));
    let mut fishline = String::from("fish");
    for f in fish::FISH {
        if g.journal.is_seen(f.id) {
            fishline.push_str(&format!(" {}:{}", f.id, g.journal.record_size(f.id)));
        }
    }
    fishline.push('\n');
    out.push_str(&fishline);
    out
}

/// Write the save atomically (temp file + rename).
pub fn save(g: &Game, path: &str) -> std::io::Result<()> {
    let tmp = format!("{}.tmp", path);
    let data = serialize(g);
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(data.as_bytes())?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    Ok(())
}

/// Parse a save into a fresh Game (seeded for its RNG, which is not persisted).
/// Returns None if the version tag is missing/unsupported or a field is corrupt.
pub fn deserialize(data: &str, seed: u64) -> Option<Game> {
    let mut g = Game::with_seed(seed);
    let mut saw_version = false;
    for line in data.lines() {
        let mut it = line.splitn(2, ' ');
        let key = it.next().unwrap_or("");
        let rest = it.next().unwrap_or("").trim();
        match key {
            "stillwater" => {
                if rest.parse::<u32>().ok()? != VERSION {
                    return None;
                }
                saw_version = true;
            }
            "year" => g.calendar.year = rest.parse().ok()?,
            "season" => g.calendar.season = u8_to_season(rest.parse().ok()?),
            "day" => g.calendar.day = rest.parse().ok()?,
            "minutes" => g.clock.minutes = rest.parse().ok()?,
            "weather" => g.weather = u8_to_weather(rest.parse().ok()?),
            "weathernext" => g.weather_next = u8_to_weather(rest.parse().ok()?),
            "px" => g.player.pos.x = rest.parse().ok()?,
            "py" => g.player.pos.y = rest.parse().ok()?,
            "energy" => g.player.energy = rest.parse().ok()?,
            "gold" => g.player.gold = rest.parse().ok()?,
            "rod" => g.player.rod_tier = rest.parse().ok()?,
            "bait" => g.player.bait_id = rest.parse().ok()?,
            "area" => g.world.set_current(rest.parse().ok()?),
            "settings" => {
                let v: Vec<&str> = rest.split_whitespace().collect();
                if v.len() >= 3 {
                    g.settings.hints = v[0] == "1";
                    g.settings.color = v[1] == "1";
