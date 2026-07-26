//! The fishing loop: cast -> wait for a bite -> hook -> reel fight -> land.
//! Pure and deterministic given an `Rng`, so the whole thing is testable headless.

use crate::fish::{self, Catch, FishDef};
use crate::rng::Rng;
use crate::season::{Season, TimeOfDay, WaterType};
use crate::weather::Weather;

/// Everything the bite/fight math needs from the wider game.
#[derive(Clone, Copy)]
pub struct BiteCtx {
    pub season: Season,
    pub tod: TimeOfDay,
    pub weather: Weather,
    pub bait_id: u16,
    pub bait_bonus: i32,
    pub rod_bonus: i32,
    pub line_strength: u8,
}

/// The active reel fight against a hooked fish.
pub struct Fight {
    pub fish_id: u16,
    pub progress: i32, // 0..=100 -> lands (or clears a boss phase)
    pub slack: i32,    // 0..=100 -> line snaps
    pub darting: bool, // fish is pulling hard right now (reel = lots of slack)
    pub difficulty: u8,
    pub line_strength: u8,
    pub boss: bool,       // a legendary giant: a multi-phase fight
    pub phases_left: u8,  // boss phases remaining (1 = final)
    pub surge: bool,      // just cleared a phase — the giant surges
}

