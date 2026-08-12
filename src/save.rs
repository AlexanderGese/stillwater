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

