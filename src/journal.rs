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

    /// Total number of species in the game.
    pub fn total_species(&self) -> usize {
        fish::FISH.len()
    }

    pub fn caught_total(&self) -> u32 {
        self.caught_total
    }

    pub fn is_complete(&self) -> bool {
        self.seen_count() == self.total_species()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_journal_empty() {
        let journal = Journal::new();
        assert_eq!(journal.seen_count(), 0);
        assert!(!journal.is_complete());
    }

    #[test]
    fn record_catch_marks_species_seen() {
        let mut journal = Journal::new();
        let catch = Catch { fish_id: 1, size: 12 };
        journal.record_catch(&catch);

        assert!(journal.is_seen(1));
        assert_eq!(journal.record_size(1), 12);
        assert_eq!(journal.seen_count(), 1);
        assert_eq!(journal.caught_total(), 1);
    }

    #[test]
    fn record_catch_keeps_largest_size() {
        let mut journal = Journal::new();

        journal.record_catch(&Catch { fish_id: 2, size: 15 });
        assert_eq!(journal.record_size(2), 15);

        journal.record_catch(&Catch { fish_id: 2, size: 20 });
        assert_eq!(journal.record_size(2), 20);

        journal.record_catch(&Catch { fish_id: 2, size: 10 });
        assert_eq!(journal.record_size(2), 20);
    }

    #[test]
    fn caught_total_counts_all_catches() {
        let mut journal = Journal::new();
        journal.record_catch(&Catch { fish_id: 1, size: 10 });
        journal.record_catch(&Catch { fish_id: 1, size: 12 });
        journal.record_catch(&Catch { fish_id: 2, size: 18 });

        assert_eq!(journal.caught_total(), 3);
    }

    #[test]
    fn out_of_range_id_ignored_safely() {
        let mut journal = Journal::new();
        // Should not panic
        journal.record_catch(&Catch { fish_id: 9999, size: 50 });

        assert!(!journal.is_seen(9999));
        assert_eq!(journal.record_size(9999), 0);
        assert_eq!(journal.caught_total(), 0);
    }

    #[test]
    fn multiple_species_tracked() {
        let mut journal = Journal::new();
        journal.record_catch(&Catch { fish_id: 1, size: 10 });
        journal.record_catch(&Catch { fish_id: 5, size: 15 });
        journal.record_catch(&Catch { fish_id: 10, size: 60 });

        assert_eq!(journal.seen_count(), 3);
        assert!(journal.is_seen(1));
        assert!(journal.is_seen(5));
        assert!(journal.is_seen(10));
        assert!(!journal.is_seen(2));
    }

    #[test]
    fn total_species_matches_fish_registry() {
        let journal = Journal::new();
        assert_eq!(journal.total_species(), fish::FISH.len());
    }

    #[test]
    fn is_complete_when_all_species_seen() {
        let mut journal = Journal::new();
        let total = journal.total_species();

        assert!(!journal.is_complete());

        // Record one catch for each species (using ids 1 through total)
        for id in 1..=total as u16 {
            journal.record_catch(&Catch { fish_id: id, size: 20 });
        }

        assert_eq!(journal.seen_count(), total);
        assert!(journal.is_complete());
    }

    #[test]
    fn is_seen_boundary_checks() {
        let journal = Journal::new();

        // Id 0 is unused (never set to true)
        assert!(!journal.is_seen(0));

        // Valid id but not yet seen
        assert!(!journal.is_seen(1));

        // Way out of range
        assert!(!journal.is_seen(9999));
    }

