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

