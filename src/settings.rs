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

