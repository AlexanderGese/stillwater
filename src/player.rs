use crate::geom::{Dir, Point};

pub const MAX_ENERGY: i32 = 100;

pub struct Player {
    pub pos: Point,
    pub facing: Dir,
    pub energy: i32,
    pub gold: u32,
    pub rod_tier: u8, // index into tackle::RODS
    pub bait_id: u16, // equipped bait id (0 = none)
}

impl Player {
    pub fn new(pos: Point) -> Player {
        Player {
            pos,
            facing: Dir::South,
            energy: MAX_ENERGY,
            gold: 0,
            rod_tier: 0,
            bait_id: 0,
        }
    }
}
