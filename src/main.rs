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

fn run(out: &mut io::Stdout) -> io::Result<()> {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1);
    let mut g = save::load(SAVE_PATH, seed).unwrap_or_else(|| game::Game::with_seed(seed));
    g.to_menu(); // always open on the main menu ("Continue" appears if a save loaded)
    while g.running {
        // draw
        let mut buf = Vec::new();
        render::draw(&g, &mut buf);
        execute!(out, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0))?;
        // raw mode needs explicit CR+LF: replace \n with \r\n
        let text = String::from_utf8_lossy(&buf).replace('\n', "\r\n");
        out.write_all(text.as_bytes())?;
        out.flush()?;

        // input (blocking)
        if let Event::Key(k) = event::read()? {
            let action = input::key_to_action(k.code).unwrap_or(game::Action::Any);
            g.apply(action);
            let _ = save::save(&g, SAVE_PATH);
        }
    }
    let _ = save::save(&g, SAVE_PATH);
    Ok(())
}
