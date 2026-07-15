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
