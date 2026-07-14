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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stepping_moves_one_tile() {
        let p = Point::new(3, 3);
        assert_eq!(p.step(Dir::North), Point::new(3, 2));
        assert_eq!(p.step(Dir::South), Point::new(3, 4));
        assert_eq!(p.step(Dir::East), Point::new(4, 3));
        assert_eq!(p.step(Dir::West), Point::new(2, 3));
    }
}
