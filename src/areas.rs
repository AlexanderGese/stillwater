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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point;
    use crate::tile::Tile;
    #[test]
    fn parses_dimensions_and_tiles() {
        let a = parse_area("t", Point::new(1, 1), "###\n#.~\n###");
        assert_eq!(a.map.w, 3);
        assert_eq!(a.map.h, 3);
        assert_eq!(a.map.get(Point::new(0, 0)), Tile::Wall);
        assert_eq!(a.map.get(Point::new(1, 1)), Tile::Grass);
        assert_eq!(a.map.get(Point::new(2, 1)), Tile::ShallowWater);
    }
    #[test]
    fn dock_area_is_valid() {
        let a = dock_area();
        assert!(a.map.w >= 10 && a.map.h >= 6);
        assert!(a.map.walkable(a.start)); // player starts on walkable ground
        // there is at least one water tile to fish later
        let mut has_water = false;
        for y in 0..a.map.h {
            for x in 0..a.map.w {
                if a.map.get(Point::new(x, y)).is_water() {
                    has_water = true;
                }
            }
        }
        assert!(has_water);
    }
}
