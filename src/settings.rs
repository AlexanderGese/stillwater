//! Player-toggleable options, persisted in the save file.

#[derive(Clone, Copy)]
pub struct Settings {
    pub hints: bool, // show the context hint line + guided-tutorial prompts
    pub color: bool, // colorize the map
    pub guide: bool, // run the step-by-step tutorial on a new game
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            hints: true,
            color: true,
            guide: true,
        }
    }

    pub const LABELS: [&'static str; 3] =
        ["Hints & tips", "Color", "Guided tutorial (new game)"];

    pub fn count() -> usize {
        Self::LABELS.len()
    }

    pub fn get(&self, i: usize) -> bool {
        match i {
            0 => self.hints,
            1 => self.color,
            2 => self.guide,
            _ => false,
        }
    }

    pub fn toggle(&mut self, i: usize) {
        match i {
            0 => self.hints = !self.hints,
            1 => self.color = !self.color,
            2 => self.guide = !self.guide,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toggles_round_trip() {
        let mut s = Settings::new();
        assert!(s.get(1));
        s.toggle(1);
        assert!(!s.get(1));
        s.toggle(1);
        assert!(s.get(1));
        assert_eq!(Settings::count(), 3);
    }
}
