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

