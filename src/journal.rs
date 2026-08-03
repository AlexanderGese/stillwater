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

    /// Record a landed fish: mark its species seen, bump the record if larger,
    /// and increment the lifetime catch count. AGENT: implement (guard the id
    /// against the vec length; never panic on an unknown id).
    pub fn record_catch(&mut self, c: &Catch) {
        let id = c.fish_id as usize;
        if id < self.seen.len() {
            self.seen[id] = true;
            if c.size > self.record[id] {
                self.record[id] = c.size;
            }
            self.caught_total += 1;
        }
    }

    pub fn is_seen(&self, id: u16) -> bool {
        let idx = id as usize;
        idx < self.seen.len() && self.seen[idx]
    }

    pub fn record_size(&self, id: u16) -> u16 {
        let idx = id as usize;
        if idx < self.record.len() {
            self.record[idx]
        } else {
            0
        }
    }

    /// Distinct species seen so far.
    pub fn seen_count(&self) -> usize {
        self.seen.iter().filter(|&&s| s).count()
    }

