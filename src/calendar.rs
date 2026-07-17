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

