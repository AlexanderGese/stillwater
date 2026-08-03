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

