#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    pub fn step(self, d: Dir) -> Point {
        match d {
            Dir::North => Point::new(self.x, self.y - 1),
            Dir::South => Point::new(self.x, self.y + 1),
            Dir::East => Point::new(self.x + 1, self.y),
            Dir::West => Point::new(self.x - 1, self.y),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

