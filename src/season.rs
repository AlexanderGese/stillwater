//! Shared calendar/atmosphere types used across the game.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    /// Days per season.
    pub const DAYS: i32 = 28;

    pub fn name(self) -> &'static str {
        match self {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Fall => "Fall",
            Season::Winter => "Winter",
        }
    }

    pub fn next(self) -> Season {
        match self {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Fall,
            Season::Fall => Season::Winter,
            Season::Winter => Season::Spring,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Season::Spring => 0,
            Season::Summer => 1,
            Season::Fall => 2,
            Season::Winter => 3,
        }
    }

    pub fn all() -> [Season; 4] {
        [Season::Spring, Season::Summer, Season::Fall, Season::Winter]
    }
}

/// Coarse time-of-day bucket, used by fishing tables and rendering mood.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TimeOfDay {
    Dawn,    // 6-8
    Morning, // 8-12
    Day,     // 12-17
    Dusk,    // 17-20
    Night,   // 20-2
}

/// Kind of water a fishing spot sits on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WaterType {
    Shallow,
    Deep,
    River,
    Marsh,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn season_cycles() {
        assert_eq!(Season::Spring.next(), Season::Summer);
        assert_eq!(Season::Winter.next(), Season::Spring);
        assert_eq!(Season::all().len(), 4);
        assert_eq!(Season::Fall.index(), 2);
    }
}
