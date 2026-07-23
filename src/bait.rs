//! Bait registry. CONTRACT STUB — an implementer agent populates `BAIT`
//! (~5-7 baits) and tests. Keep the public types/signatures.

#[derive(Clone, Copy, Debug)]
pub struct BaitDef {
    pub id: u16,
    pub name: &'static str,
    pub cost: u32,
    /// Percent bite-chance bonus this bait grants.
    pub bite_bonus: i32,
}

/// All baits. AGENT: populate. Include a free/basic bait (e.g. Worm, cost 0 or
/// cheap) and a couple of better ones. Ids unique and > 0.
pub static BAIT: &[BaitDef] = &[
    BaitDef {
        id: 1,
        name: "Worm",
        cost: 5,
        bite_bonus: 5,
    },
    BaitDef {
        id: 2,
        name: "Grub",
        cost: 12,
        bite_bonus: 10,
    },
    BaitDef {
        id: 3,
        name: "Cricket",
        cost: 20,
        bite_bonus: 15,
    },
    BaitDef {
