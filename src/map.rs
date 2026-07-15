use crate::geom::Point;
use crate::tile::Tile;

pub struct Map {
    pub w: i32,
    pub h: i32,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(w: i32, h: i32) -> Map {
        Map {
            w,
            h,
            tiles: vec![Tile::Grass; (w * h) as usize],
        }
    }
    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.w && p.y < self.h
    }
    fn idx(&self, p: Point) -> usize {
        (p.y * self.w + p.x) as usize
    }
    pub fn get(&self, p: Point) -> Tile {
        if self.in_bounds(p) {
            self.tiles[self.idx(p)]
        } else {
            Tile::Wall
        }
    }
    pub fn set(&mut self, p: Point, t: Tile) {
        if self.in_bounds(p) {
            let i = self.idx(p);
            self.tiles[i] = t;
        }
    }
    pub fn walkable(&self, p: Point) -> bool {
        self.in_bounds(p) && self.get(p).walkable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point;
    use crate::tile::Tile;
    #[test]
    fn set_get_and_bounds() {
        let mut m = Map::new(5, 4);
        assert_eq!(m.get(Point::new(0, 0)), Tile::Grass);
        m.set(Point::new(2, 1), Tile::Wall);
        assert_eq!(m.get(Point::new(2, 1)), Tile::Wall);
        assert!(!m.walkable(Point::new(2, 1)));
        assert!(m.walkable(Point::new(0, 0)));
        assert_eq!(m.get(Point::new(-1, 0)), Tile::Wall); // oob
        assert!(!m.in_bounds(Point::new(5, 0)));
        assert!(!m.walkable(Point::new(5, 0)));
    }
}
