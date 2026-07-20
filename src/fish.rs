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
        glyph: 'm',
        seasons: &[Spring, Summer, Fall],
        waters: &[Marsh],
        times: ALL_TIMES,
        rarity: 1,
        size_min: 6,
        size_max: 12,
        difficulty: 1,
        base_price: 3,
        bait_pref: 0,
    },
    FishDef {
        id: 4,
        name: "Blossom Trout",
        glyph: 't',
        seasons: &[Spring],
        waters: &[River],
        times: &[Dawn, Dusk],
        rarity: 2,
        size_min: 20,
        size_max: 38,
        difficulty: 4,
        base_price: 18,
        bait_pref: 3, // Cricket
    },
    FishDef {
        id: 5,
        name: "Speckled Dace",
        glyph: 'd',
        seasons: &[Spring, Fall],
        waters: &[River, Shallow],
        times: &[Morning, Day],
        rarity: 1,
        size_min: 9,
        size_max: 18,
        difficulty: 2,
        base_price: 5,
        bait_pref: 0,
    },
    FishDef {
        id: 6,
        name: "Frogmouth Bass",
        glyph: 'b',
        seasons: &[Spring, Summer],
        waters: &[Marsh, Shallow],
        times: &[Dusk, Night],
        rarity: 2,
        size_min: 24,
        size_max: 45,
        difficulty: 5,
        base_price: 22,
        bait_pref: 2, // Grub
    },
    FishDef {
        id: 7,
        name: "Emerald Chub",
        glyph: 'c',
        seasons: &[Spring, Summer, Fall],
        waters: &[Shallow, River],
        times: ALL_TIMES,
        rarity: 1,
        size_min: 10,
        size_max: 20,
        difficulty: 1,
        base_price: 5,
        bait_pref: 0,
    },
    // --- Summer ---
    FishDef {
        id: 8,
        name: "Golden Carp",
        glyph: 'C',
        seasons: &[Summer, Fall],
        waters: &[Deep, Shallow],
        times: &[Day, Dusk],
        rarity: 2,
        size_min: 30,
        size_max: 60,
        difficulty: 5,
        base_price: 28,
        bait_pref: 1,
    },
    FishDef {
        id: 9,
        name: "Sunburst Bluegill",
        glyph: 'g',
        seasons: &[Summer],
        waters: &[Shallow],
        times: &[Morning, Day],
        rarity: 1,
        size_min: 8,
        size_max: 15,
        difficulty: 1,
        base_price: 4,
        bait_pref: 1,
    },
    FishDef {
        id: 10,
        name: "River Pike",
        glyph: 'P',
        seasons: &[Summer, Fall],
        waters: &[River, Deep],
