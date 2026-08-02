use crate::area_deeplake::deeplake_area;
use crate::area_homestead::homestead_area;
use crate::area_lake::lake_area;
use crate::area_marsh::marsh_area;
use crate::area_river::river_area;
use crate::area_town::town_area;
use crate::areas::Area;
use crate::geom::{Dir, Point};
use crate::map::Map;
use crate::restore;
use crate::season::WaterType;

// Area indices.
pub const HOMESTEAD: usize = 0;
pub const LAKE: usize = 1;
pub const TOWN: usize = 2;
pub const RIVER: usize = 3;
pub const MARSH: usize = 4;
pub const DEEPLAKE: usize = 5;

/// A directional link off a map edge to another area, optionally gated behind a
/// restoration project.
#[derive(Clone, Copy)]
pub struct Exit {
    pub dir: Dir,
    pub to: usize,
    pub project: Option<usize>,
}

