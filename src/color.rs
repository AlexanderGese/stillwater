//! Minimal ANSI colour for the map. Codes are written straight into the render
//! buffer; the terminal interprets them and tests ignore them. Only used when
//! `settings.color` is on.

use crate::tile::Tile;

pub const RESET: &str = "\x1b[0m";
pub const PLAYER: &str = "\x1b[1;93m"; // bright yellow

/// Foreground colour code for a tile's glyph.
pub fn tile(t: Tile) -> &'static str {
    match t {
        Tile::Grass => "\x1b[32m",         // green
        Tile::Path => "\x1b[33m",          // dull yellow
        Tile::Wall => "\x1b[90m",          // grey
        Tile::ShallowWater => "\x1b[96m",  // bright cyan
        Tile::DeepWater => "\x1b[34m",     // blue
        Tile::Dock | Tile::Bridge => "\x1b[33m", // wood yellow
        Tile::Bed => "\x1b[95m",           // magenta
        Tile::ShipBin => "\x1b[93m",       // bright yellow
        Tile::Shop => "\x1b[92m",          // bright green
        Tile::Sign => "\x1b[93m",          // bright yellow
        Tile::Tree => "\x1b[32m",          // green
        Tile::Rock => "\x1b[90m",          // grey
    }
}
