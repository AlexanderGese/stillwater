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

pub struct World {
    areas: Vec<Area>,
    exits: Vec<Vec<Exit>>,
    funded: Vec<bool>,
    current: usize,
}

impl World {
    pub fn new() -> World {
        let areas = vec![
            homestead_area(),
            lake_area(),
            town_area(),
            river_area(),
            marsh_area(),
            deeplake_area(),
        ];
        let e = |dir, to, project| Exit { dir, to, project };
        let exits = vec![
            // HOMESTEAD (hub)
            vec![
                e(Dir::East, LAKE, None),
                e(Dir::North, TOWN, None),
                e(Dir::West, MARSH, Some(0)), // Clear the Reeds
            ],
            // LAKE
            vec![
                e(Dir::West, HOMESTEAD, None),
                e(Dir::North, RIVER, Some(1)),   // Mend the River Path
                e(Dir::East, DEEPLAKE, Some(2)), // Repair the Rowboat
            ],
            // TOWN
            vec![e(Dir::South, HOMESTEAD, None)],
            // RIVER
            vec![e(Dir::South, LAKE, Some(1))],
            // MARSH
            vec![e(Dir::East, HOMESTEAD, Some(0))],
            // DEEPLAKE
            vec![e(Dir::West, LAKE, Some(2))],
        ];
        World {
            areas,
            exits,
            funded: vec![false; restore::PROJECTS.len()],
            current: HOMESTEAD,
        }
    }

    pub fn area(&self) -> &Area {
        &self.areas[self.current]
    }
    pub fn map(&self) -> &Map {
        &self.areas[self.current].map
    }
    pub fn area_name(&self) -> &str {
        &self.areas[self.current].name
    }
    pub fn current(&self) -> usize {
        self.current
    }
    pub fn set_current(&mut self, i: usize) {
        if i < self.areas.len() {
            self.current = i;
        }
    }
    pub fn water_kind(&self) -> Option<WaterType> {
        self.areas[self.current].water_kind
    }

    pub fn is_funded(&self, project: usize) -> bool {
        project < self.funded.len() && self.funded[project]
    }
    pub fn fund(&mut self, project: usize) {
        if project < self.funded.len() {
            self.funded[project] = true;
        }
    }

    /// Resolve an attempt to leave the current area heading `dir`.
    /// Returns None if there's no exit that way; Ok((area, entry)) if it's open;
    /// Err(project name) if a restoration project still gates it.
    pub fn exit_toward(&self, dir: Dir) -> Option<Result<(usize, Point), &'static str>> {
        let exit = *self.exits[self.current].iter().find(|e| e.dir == dir)?;
        match exit.project {
            Some(p) if !self.is_funded(p) => {
                Some(Err(restore::project(p).map(|d| d.name).unwrap_or("???")))
            }
            _ => {
                let entry = entry_point(&self.areas[exit.to].map, opposite(dir));
                Some(Ok((exit.to, entry)))
            }
        }
    }
}

fn opposite(d: Dir) -> Dir {
    match d {
        Dir::North => Dir::South,
        Dir::South => Dir::North,
        Dir::East => Dir::West,
        Dir::West => Dir::East,
    }
}

/// Find a walkable tile to drop the player on when they arrive at a map's `side`
/// edge. Scans the edge for the walkable tile nearest the center.
fn entry_point(map: &Map, side: Dir) -> Point {
    let (w, h) = (map.w, map.h);
    let cy = h / 2;
    let cx = w / 2;
    // walkable tile on column `x` nearest the vertical center
    let col_near = |x: i32| -> Option<Point> {
        (0..h).find_map(|dy| {
            [cy + dy, cy - dy]
                .into_iter()
                .find(|&y| map.walkable(Point::new(x, y)))
                .map(|y| Point::new(x, y))
        })
    };
    // walkable tile on row `y` nearest the horizontal center
    let row_near = |y: i32| -> Option<Point> {
        (0..w).find_map(|dx| {
            [cx + dx, cx - dx]
                .into_iter()
                .find(|&x| map.walkable(Point::new(x, y)))
                .map(|x| Point::new(x, y))
        })
    };
    let found = match side {
        Dir::West => (0..w).find_map(col_near),
        Dir::East => (0..w).rev().find_map(col_near),
        Dir::North => (0..h).find_map(row_near),
        Dir::South => (0..h).rev().find_map(row_near),
    };
    found.unwrap_or(Point::new(cx, cy))
}

