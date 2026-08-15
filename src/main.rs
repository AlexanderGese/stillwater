// The content registries and engine expose a deliberately broad API surface
// (fish/bait/tackle fields, weather/season helpers, test-only sim harness) that
// not every path consumes yet; silence dead-code noise crate-wide.
#![allow(dead_code)]

mod area_deeplake;
mod area_homestead;
mod area_lake;
mod area_marsh;
mod area_river;
mod area_town;
mod areas;
mod bait;
mod calendar;
mod clock;
mod color;
mod fish;
mod fishing;
mod flavor;
mod game;
mod geom;
mod input;
mod journal;
mod map;
mod player;
mod render;
mod restore;
mod rng;
mod save;
mod season;
mod settings;
mod shop;
mod story;
mod sim;
mod tackle;
mod tile;
mod tutorial;
mod weather;
mod world;

use crossterm::{
    cursor,
    event::{self, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut out = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, cursor::Hide)?;

    let result = run(&mut out);

    execute!(out, cursor::Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    result
}

const SAVE_PATH: &str = ".stillwater.save";

