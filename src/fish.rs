//! Fish species registry + a caught-fish instance.

use crate::season::{Season, TimeOfDay, WaterType};

#[derive(Clone, Copy, Debug)]
pub struct FishDef {
    pub id: u16,
    pub name: &'static str,
    pub glyph: char,
    pub seasons: &'static [Season],
    pub waters: &'static [WaterType],
    pub times: &'static [TimeOfDay],
    pub rarity: u8,       // 1 common .. 5 legendary
    pub size_min: u16,    // cm
    pub size_max: u16,    // cm (>= size_min)
    pub difficulty: u8,   // 1..10 reel-fight difficulty
    pub base_price: u32,  // gold at the average size
    pub bait_pref: u16,   // preferred bait id; 0 = no preference
}

use Season::*;
use TimeOfDay::*;
use WaterType::*;

const ALL_SEASONS: &[Season] = &[Spring, Summer, Fall, Winter];
const ALL_TIMES: &[TimeOfDay] = &[Dawn, Morning, Day, Dusk, Night];
const ALL_WATERS: &[WaterType] = &[Shallow, Deep, River, Marsh];

/// The whole species list. Ids must be unique and > 0.
pub static FISH: &[FishDef] = &[
    // --- Spring ---
    FishDef {
        id: 1,
        name: "Sunfin Panfish",
        glyph: 'p',
        seasons: &[Spring, Summer],
        waters: &[Shallow],
        times: &[Dawn, Morning, Day],
        rarity: 1,
        size_min: 8,
        size_max: 16,
        difficulty: 1,
        base_price: 4,
        bait_pref: 1, // Worm
    },
    FishDef {
        id: 2,
        name: "Spring Roach",
        glyph: 'r',
        seasons: &[Spring],
        waters: &[Shallow, River],
        times: &[Dawn, Morning],
        rarity: 1,
        size_min: 10,
        size_max: 22,
        difficulty: 2,
        base_price: 6,
        bait_pref: 1,
    },
    FishDef {
        id: 3,
        name: "Marsh Minnow",
