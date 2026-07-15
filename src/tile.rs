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

impl Tile {
    pub fn walkable(self) -> bool {
        matches!(
            self,
            Tile::Grass | Tile::Path | Tile::Dock | Tile::Bed | Tile::ShipBin | Tile::Bridge
        )
    }
    pub fn is_water(self) -> bool {
        matches!(self, Tile::ShallowWater | Tile::DeepWater)
    }
    pub fn glyph(self) -> char {
        match self {
            Tile::Grass => '.',
            Tile::Path => ',',
            Tile::Wall => '#',
            Tile::ShallowWater => '~',
            Tile::DeepWater => '\u{2248}', // ≈
            Tile::Dock => '=',
            Tile::Bed => 'B',
            Tile::ShipBin => 'S',
            Tile::Shop => 'H',
            Tile::Sign => 'N',
            Tile::Tree => '\u{2663}', // ♣
            Tile::Rock => 'o',
            Tile::Bridge => '\u{2550}', // ═ (a plank crossing)
        }
    }
    pub fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Wall,
            ',' => Tile::Path,
            '~' => Tile::ShallowWater,
            '\u{2248}' => Tile::DeepWater,
            '=' => Tile::Dock,
