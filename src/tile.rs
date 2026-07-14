#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Grass,
    Path,
    Wall,
    ShallowWater,
    DeepWater,
    Dock,
    Bed,
    ShipBin,
    Shop,
    Sign, // restoration notice board
    Tree,
    Rock,
    Bridge, // walkable crossing over water
}

