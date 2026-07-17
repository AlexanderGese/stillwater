//! The date: year / season / day-of-season (1..=28).

use crate::season::Season;

pub struct Calendar {
    pub year: i32,
    pub season: Season,
    pub day: i32, // 1..=Season::DAYS
}

impl Calendar {
    pub fn new() -> Calendar {
        Calendar {
            year: 1,
            season: Season::Spring,
            day: 1,
        }
    }

    /// Advance to the next day, rolling season (and year) at the boundary.
    /// Returns true if the season changed.
    pub fn advance_day(&mut self) -> bool {
        self.day += 1;
        if self.day > Season::DAYS {
            self.day = 1;
            self.season = self.season.next();
            if self.season == Season::Spring {
                self.year += 1;
            }
            true
        } else {
            false
        }
    }

    /// "Spring 3" style label.
    pub fn label(&self) -> String {
        format!("{} {}", self.season.name(), self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rolls_season_and_year() {
        let mut c = Calendar::new();
        assert_eq!(c.label(), "Spring 1");
        for _ in 0..27 {
            assert!(!c.advance_day());
        }
        assert_eq!(c.day, 28);
        assert!(c.advance_day()); // -> Summer 1
        assert_eq!(c.season, Season::Summer);
        assert_eq!(c.day, 1);
        // roll all the way to next year
        let mut c = Calendar::new();
        for _ in 0..(Season::DAYS * 4) {
            c.advance_day();
        }
        assert_eq!(c.year, 2);
        assert_eq!(c.season, Season::Spring);
        assert_eq!(c.day, 1);
    }
}
