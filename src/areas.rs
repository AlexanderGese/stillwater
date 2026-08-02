use crate::geom::Point;
use crate::map::Map;
use crate::season::WaterType;
use crate::tile::Tile;

pub struct Area {
    pub name: String,
    pub map: Map,
    pub start: Point,
    /// If set, ALL water tiles in this area count as this kind of water for
    /// fishing (e.g. a River or Marsh). If None, water is typed per-tile
    /// (ShallowWater -> Shallow, DeepWater -> Deep).
    pub water_kind: Option<WaterType>,
}

pub fn parse_area(name: &str, start: Point, template: &str) -> Area {
    parse_area_water(name, start, template, None)
}

