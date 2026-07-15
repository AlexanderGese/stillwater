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
            'B' => Tile::Bed,
            'S' => Tile::ShipBin,
            'H' => Tile::Shop,
            'N' => Tile::Sign,
            'T' => Tile::Tree,
            'o' => Tile::Rock,
            'x' => Tile::Bridge,
            _ => Tile::Grass,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn walkability_and_water() {
        assert!(Tile::Grass.walkable());
        assert!(Tile::Dock.walkable());
        assert!(!Tile::Wall.walkable());
        assert!(!Tile::ShallowWater.walkable());
        assert!(Tile::ShallowWater.is_water());
        assert!(Tile::DeepWater.is_water());
        assert!(!Tile::Grass.is_water());
    }
    #[test]
    fn from_char_roundtrips_key_tiles() {
        assert_eq!(Tile::from_char('#'), Tile::Wall);
        assert_eq!(Tile::from_char('~'), Tile::ShallowWater);
        assert_eq!(Tile::from_char('='), Tile::Dock);
        assert_eq!(Tile::from_char('.'), Tile::Grass);
        assert_eq!(Tile::from_char('?'), Tile::Grass); // unknown fallback
    }
}
