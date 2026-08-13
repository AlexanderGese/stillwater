use crate::fish;
use crate::fishing::{Phase, Session};
use crate::game::{Game, Mode};
use crate::geom::Point;
use crate::map::Map;
use crate::player::MAX_ENERGY;
use crate::restore;
use crate::shop;
use crate::tile::Tile;
use std::io::Write;

/// Number of header lines drawn above the map.
pub const HUD_LINES: usize = 2;

/// Viewport size — the window onto the (possibly larger) map, centered on you.
const VIEW_W: i32 = 78;
const VIEW_H: i32 = 26;

pub fn draw(g: &Game, buf: &mut Vec<u8>) {
    match &g.mode {
        Mode::Menu { sel } => {
            draw_menu(g, *sel, buf);
            return;
        }
        Mode::Settings { sel } => {
            draw_settings(g, *sel, buf);
            return;
        }
        Mode::Story { page } => {
            draw_story(g, *page, buf);
            return;
        }
        _ => {}
    }
    draw_header(g, buf);
    match &g.mode {
        Mode::Explore => {
            draw_map(g, buf);
            if !g.message.is_empty() {
                let _ = writeln!(buf, "{}", g.message);
            }
            let _ = writeln!(
                buf,
                "[wasd] move  [e] cast/act  [j] journal  [z] wait  [?] help  [q] quit"
            );
            draw_guide_or_hint(g, buf);
        }
        Mode::Fishing(s) => {
            draw_map(g, buf);
            draw_fishing(s, buf);
            draw_guide_or_hint(g, buf);
        }
