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

    /// Hour in 0..24 (wrapping past midnight).
    pub fn hour24(&self) -> i32 {
        (self.minutes / 60) % 24
    }

    pub fn minute(&self) -> i32 {
        self.minutes % 60
    }

    pub fn is_past(&self, hour24: i32) -> bool {
        self.minutes >= hour24 * 60
    }

    pub fn should_collapse(&self) -> bool {
        self.minutes >= Self::COLLAPSE
    }

    pub fn is_dusk(&self) -> bool {
        self.minutes >= Self::DUSK
    }

    pub fn tod(&self) -> TimeOfDay {
        let h = self.hour24();
        match h {
            6..=7 => TimeOfDay::Dawn,
            8..=11 => TimeOfDay::Morning,
            12..=16 => TimeOfDay::Day,
            17..=19 => TimeOfDay::Dusk,
            _ => TimeOfDay::Night, // 20-23 and 0-5
        }
    }

    /// "6:00am" style label.
    pub fn label(&self) -> String {
        let h24 = self.hour24();
        let m = self.minute();
        let (h12, ampm) = if h24 == 0 {
            (12, "am")
        } else if h24 < 12 {
            (h24, "am")
        } else if h24 == 12 {
            (12, "pm")
        } else {
            (h24 - 12, "pm")
        };
        format!("{}:{:02}{}", h12, m, ampm)
    }
}

