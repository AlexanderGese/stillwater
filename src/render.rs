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

fn draw_menu(g: &Game, sel: usize, buf: &mut Vec<u8>) {
    const W: usize = 29;
    let bar: String = "\u{2550}".repeat(W);
    let _ = writeln!(buf, "\n\n");
    let _ = writeln!(buf, "        \u{2554}{}\u{2557}", bar);
    let _ = writeln!(buf, "        \u{2551}{:^W$}\u{2551}", "S T I L L W A T E R");
    let _ = writeln!(buf, "        \u{2551}{:^W$}\u{2551}", "F  A  R  M");
    let _ = writeln!(buf, "        \u{255A}{}\u{255D}", bar);
    let _ = writeln!(buf, "           a cozy fishing life\n");
    for (i, opt) in g.menu_options().iter().enumerate() {
        let cursor = if i == sel { "\u{25B8} " } else { "  " };
        let _ = writeln!(buf, "            {}{}. {}", cursor, i + 1, opt);
    }
    let _ = writeln!(buf, "\n   [w/s] choose   [enter] select   (or press the number)");
}

fn draw_story(g: &Game, page: usize, buf: &mut Vec<u8>) {
    let text = g.story_pages.get(page).copied().unwrap_or("");
    let _ = writeln!(buf, "\n");
    let _ = writeln!(buf, "{}", text);
    let total = g.story_pages.len().max(1);
    let _ = writeln!(buf, "\n                                              ({}/{})", page + 1, total);
}

fn draw_help(buf: &mut Vec<u8>) {
    let _ = writeln!(buf, "~~~~~ how to play ~~~~~");
    let lines = [
        "  move ............ W A S D  /  arrow keys",
        "  cast / act ...... E  (at water: cast; at bed: sleep; at H: shop; at N: board)",
        "  reel fight ...... W reel,  S ease  (ease when the fish is DARTING)",
        "  wait ............ Z",
        "  journal ......... J",
        "  buy / fund ...... 1-9  (in the shop or on the restoration board)",
        "  travel .......... walk into an open edge of the map",
        "  close menu ...... Esc      quit ...... Q",
        "",
        "  Catch fish for gold, buy better gear at the shop (H), sleep (B) to pass",
        "  days & seasons, and fund restoration at the town board (N) to open the",
        "  marsh, river, and deep lake.",
    ];
    for l in lines {
        let _ = writeln!(buf, "{}", l);
    }
    let _ = writeln!(buf, "[any key] close");
}


fn draw_header(g: &Game, buf: &mut Vec<u8>) {
    let _ = writeln!(buf, "Stillwater Farm  \u{2014}  {}", g.world.area_name());
    let time = if g.clock.is_dusk() {
        format!("{} (dusk)", g.clock.label())
    } else {
        g.clock.label()
    };
    let _ = writeln!(
        buf,
        "{}  {}  {} {}   energy {}   gold {}",
        g.calendar.label(),
        time,
        g.weather.glyph(),
        g.weather.name(),
        bar(g.player.energy, MAX_ENERGY, 10),
        g.player.gold
    );
}

fn draw_map(g: &Game, buf: &mut Vec<u8>) {
    let map = g.world.map();
    let color = g.settings.color;
    let vw = VIEW_W.min(map.w);
    let vh = VIEW_H.min(map.h);
    let cam_x = (g.player.pos.x - vw / 2).clamp(0, (map.w - vw).max(0));
    let cam_y = (g.player.pos.y - vh / 2).clamp(0, (map.h - vh).max(0));
    for row in 0..vh {
        let y = cam_y + row;
        let mut line = String::new();
        for col in 0..vw {
            let x = cam_x + col;
            let p = Point::new(x, y);
            if p == g.player.pos {
                if color {
                    line.push_str(crate::color::PLAYER);
                    line.push('@');
                    line.push_str(crate::color::RESET);
                } else {
                    line.push('@');
                }
            } else {
                let t = map.get(p);
                let ch = if t == Tile::Wall {
                    wall_glyph(map, p)
                } else {
                    t.glyph()
                };
                if color {
                    line.push_str(crate::color::tile(t));
                    line.push(ch);
                    line.push_str(crate::color::RESET);
                } else {
                    line.push(ch);
                }
            }
        }
        let _ = writeln!(buf, "{}", line);
    }
}

/// Smart box-drawing glyph for a wall tile, from its wall neighbours.
/// Out-of-bounds counts as wall so the map border joins up cleanly.
fn wall_glyph(map: &Map, p: Point) -> char {
    let is_wall = |x: i32, y: i32| map.get(Point::new(x, y)) == Tile::Wall;
    let mut m = 0u8;
    if is_wall(p.x, p.y - 1) {
        m |= 1;
    }
    if is_wall(p.x + 1, p.y) {
        m |= 2;
    }
    if is_wall(p.x, p.y + 1) {
        m |= 4;
    }
    if is_wall(p.x - 1, p.y) {
        m |= 8;
    }
    match m {
        1 | 4 | 5 => '\u{2502}',  // │
        2 | 8 | 10 => '\u{2500}', // ─
        6 => '\u{250C}',          // ┌
        12 => '\u{2510}',         // ┐
        3 => '\u{2514}',          // └
        9 => '\u{2518}',          // ┘
        7 => '\u{251C}',          // ├
        13 => '\u{2524}',         // ┤
        14 => '\u{252C}',         // ┬
        11 => '\u{2534}',         // ┴
        15 => '\u{253C}',         // ┼
        _ => '\u{2500}',          // ─ (isolated)
    }
}

fn draw_fishing(s: &Session, buf: &mut Vec<u8>) {
    let _ = writeln!(buf, "~~~~~ fishing ~~~~~");
    match &s.phase {
        Phase::Waiting { waited } => {
            let dots = ".".repeat((*waited as usize).min(8) + 1);
            let _ = writeln!(buf, "your line drifts on the water{}   [any key] wait", dots);
        }
        Phase::Bite { .. } => {
            let _ = writeln!(buf, "!!!  a bite!  press [e] to set the hook!");
        }
        Phase::Fighting(f) => {
            if f.boss {
                let name = fish::by_id(f.fish_id).map(|d| d.name).unwrap_or("a giant");
                let surge = if f.surge { "   \u{2014} IT SURGES!" } else { "" };
                let _ = writeln!(
                    buf,
                    "\u{2694} A LEGEND ON THE LINE: {}  \u{2014} phase {} of 3{}",
                    name, f.phases_left, surge
                );
            }
            let _ = writeln!(
                buf,
                "reel  {}  {}%",
                bar(f.progress, 100, 16),
                f.progress.clamp(0, 100)
            );
            let hint = if f.darting {
                "<< it's DARTING - ease off!"
            } else {
                "steady - reel it in!"
            };
            let _ = writeln!(buf, "line  {}  {}", bar(f.slack, 100, 16), hint);
            let _ = writeln!(buf, "[w] reel   [s] ease");
        }
        Phase::Landed(c) => {
            let name = fish::by_id(c.fish_id).map(|f| f.name).unwrap_or("fish");
            let _ = writeln!(buf, "you landed a {} ({}cm)!   [e] continue", name, c.size);
        }
        Phase::Lost(reason) => {
            let _ = writeln!(buf, "{}   [e] continue", reason);
        }
    }
}

fn draw_shop(g: &Game, buf: &mut Vec<u8>) {
    let _ = writeln!(buf, "~~~~~ tackle shop ~~~~~   gold: {}", g.player.gold);
    let offers = shop::offers(g.player.rod_tier, g.player.bait_id);
    for (i, off) in offers.iter().enumerate() {
        let tag = if off.owned { "  (equipped)" } else { "" };
        let _ = writeln!(buf, "  {}) {:<18} {}g{}", i + 1, off.name, off.price, tag);
    }
    let _ = writeln!(buf, "[1-{}] buy   [esc] leave", offers.len().min(9));
    if !g.message.is_empty() {
        let _ = writeln!(buf, "{}", g.message);
    }
}

fn draw_journal(g: &Game, buf: &mut Vec<u8>) {
    let _ = writeln!(
        buf,
        "~~~~~ fish journal ~~~~~   {}/{} species",
        g.journal.seen_count(),
        g.journal.total_species()
    );
    for f in fish::FISH {
        if g.journal.is_seen(f.id) {
            let _ = writeln!(
                buf,
                "  {} {:<20} record {}cm",
                f.glyph,
                f.name,
                g.journal.record_size(f.id)
            );
        } else {
            let _ = writeln!(buf, "  ? ???");
        }
    }
    let done = if g.journal.is_complete() {
        "   the journal is COMPLETE!"
    } else {
        ""
    };
    let _ = writeln!(buf, "caught {} fish in all.{}", g.journal.caught_total(), done);
    let _ = writeln!(buf, "[any key] close");
}

fn draw_restore(g: &Game, buf: &mut Vec<u8>) {
    let _ = writeln!(buf, "~~~~~ restoration board ~~~~~   gold: {}", g.player.gold);
    for (i, p) in restore::PROJECTS.iter().enumerate() {
        let status = if g.world.is_funded(i) {
            "DONE".to_string()
        } else {
            format!("{}g", p.cost)
        };
        let _ = writeln!(buf, "  {}) {:<22} {:<8} - {}", i + 1, p.name, status, p.blurb);
    }
    let _ = writeln!(buf, "[1-{}] fund   [esc] leave", restore::PROJECTS.len());
    if !g.message.is_empty() {
        let _ = writeln!(buf, "{}", g.message);
    }
}

/// A `[####----]` bar, `width` cells, filled proportional to `val/max`.
fn bar(val: i32, max: i32, width: usize) -> String {
    let ratio = if max > 0 {
        (val.clamp(0, max) as f32) / max as f32
    } else {
        0.0
    };
    let filled = ((ratio * width as f32).round() as usize).min(width);
    let mut s = String::from("[");
    for i in 0..width {
        s.push(if i < filled { '#' } else { '-' });
    }
    s.push(']');
    s
}

