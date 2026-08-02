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

/// Build an area from an ASCII template. Tile alphabet (see `Tile::from_char`):
/// `#` wall, `.` grass, `,` path, `~` shallow water, `≈` deep water, `=` dock,
/// `x` bridge (walkable over water), `B` bed, `S` ship bin, `H` shop,
/// `N` notice board, `T` tree, `o` rock; anything else -> grass.
pub fn parse_area_water(
    name: &str,
    start: Point,
    template: &str,
    water_kind: Option<WaterType>,
) -> Area {
    let lines: Vec<&str> = template.lines().collect();
    let h = lines.len() as i32;
    let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as i32;
    let mut map = Map::new(w, h);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.set(Point::new(x as i32, y as i32), Tile::from_char(c));
        }
    }
    Area {
        name: name.to_string(),
        map,
        start,
        water_kind,
    }
}

pub fn dock_area() -> Area {
    // A cozy lakeside: walled yard, a dock (=) reaching into shallow (~) then deep (≈) water.
    let template = "\
################\n\
#..B.........~~#\n\
#............~≈#\n\
#....,,,======~#\n\
#....,.......~≈#\n\
#..SH,.......~~#\n\
#............~~#\n\
################";
    parse_area("Dock", Point::new(3, 4), template)
}

