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
        Mode::Shop => draw_shop(g, buf),
        Mode::Journal => draw_journal(g, buf),
        Mode::Restore => draw_restore(g, buf),
        Mode::Help => draw_help(buf),
        Mode::Menu { .. } | Mode::Settings { .. } | Mode::Story { .. } => {}
    }
}

fn draw_settings(g: &Game, sel: usize, buf: &mut Vec<u8>) {
    use crate::settings::Settings;
    let _ = writeln!(buf, "\n\n");
    let _ = writeln!(buf, "        ~~~~~  S E T T I N G S  ~~~~~\n");
    for (i, label) in Settings::LABELS.iter().enumerate() {
        let cursor = if i == sel { "\u{25B8} " } else { "  " };
        let val = if g.settings.get(i) { "ON " } else { "OFF" };
        let _ = writeln!(buf, "        {}{}. [{}]  {}", cursor, i + 1, val, label);
    }
    let _ = writeln!(buf, "\n   [w/s] choose   [enter] toggle   [esc] back");
}

fn draw_guide_or_hint(g: &Game, buf: &mut Vec<u8>) {
    if let Some(s) = g.guide_step {
        let _ = writeln!(
            buf,
            "\u{2605} {}   [esc] skip",
            crate::tutorial::guide_prompt(s)
        );
    } else if g.settings.hints {
        if let Some(h) = crate::tutorial::hint(g) {
            let _ = writeln!(buf, "\u{00bb} {}", h);
        }
    }
}

