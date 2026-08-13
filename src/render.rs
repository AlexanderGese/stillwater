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

