//! The fish journal: which species you've caught and your record size for each.
//! CONTRACT STUB — an implementer agent fills the bodies + tests. Keep the
//! public types/signatures exactly (game.rs and render.rs call these).

use crate::fish::{self, Catch};

pub struct Journal {
    seen: Vec<bool>,  // indexed by fish id (index 0 unused)
    record: Vec<u16>, // record size (cm) per fish id
    caught_total: u32,
}

impl Journal {
    /// Sized to hold every fish id in `fish::FISH` (max id + 1).
    pub fn new() -> Journal {
        let max_id = fish::FISH.iter().map(|f| f.id).max().unwrap_or(0) as usize;
        Journal {
            seen: vec![false; max_id + 1],
            record: vec![0; max_id + 1],
            caught_total: 0,
        }
    }

