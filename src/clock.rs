//! In-game time of day. Minutes are counted from midnight; a day starts at
//! 6:00am and the player passes out if still awake at 2:00am (26:00).

use crate::season::TimeOfDay;

pub struct Clock {
    /// Minutes since midnight. Starts at DAY_START; can run past 24h into the
    /// small hours (e.g. 25*60 = 1am) until COLLAPSE forces sleep.
    pub minutes: i32,
}

impl Clock {
    pub const DAY_START: i32 = 6 * 60; // 6:00am
    pub const COLLAPSE: i32 = 26 * 60; // 2:00am next day
    pub const SHOP_CLOSE: i32 = 17 * 60; // 5:00pm
    pub const DUSK: i32 = 18 * 60; // 6:00pm

    pub fn new() -> Clock {
        Clock {
            minutes: Self::DAY_START,
        }
    }

    pub fn advance(&mut self, mins: i32) {
        self.minutes += mins;
    }

    pub fn reset_morning(&mut self) {
        self.minutes = Self::DAY_START;
    }

