//! Restoration projects — the spine. Fund them with gold at the town notice
//! board to open new waters. CONTRACT STUB: an agent fills the 3 projects.
//!
//! The project INDEX carries meaning (world.rs wires each to an area unlock):
//!   index 0 -> opens The Marsh   (cheapest)
//!   index 1 -> opens The River   (mid)
//!   index 2 -> opens The Deep Lake (priciest)

#[derive(Clone, Copy, Debug)]
pub struct ProjectDef {
    pub name: &'static str,
    pub blurb: &'static str,
    pub cost: u32,
}

/// Exactly 3 projects, in the index order documented above.
pub static PROJECTS: &[ProjectDef] = &[
    ProjectDef {
        name: "Clear the Reeds",
        blurb: "opens the way to the marsh",
        cost: 300,
    },
    ProjectDef {
        name: "Mend the River Path",
        blurb: "opens the way to the river",
        cost: 1500,
    },
    ProjectDef {
        name: "Repair the Rowboat",
        blurb: "opens the way to the deep lake",
        cost: 5000,
    },
];

