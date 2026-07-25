//! Rod/tackle tiers. CONTRACT STUB — an implementer agent populates `RODS`
//! (~4 tiers) and tests. Keep the public types/signatures.

#[derive(Clone, Copy, Debug)]
pub struct RodDef {
    pub tier: u8,          // 0 = starter
    pub name: &'static str,
    pub reach: i32,        // how many water tiles ahead you can cast
    pub bite_bonus: i32,   // percent bite-chance bonus
    pub line_strength: u8, // widens the reel safe-band (fight forgiveness)
    pub cost: u32,         // shop price (tier 0 = 0, you start with it)
}

/// Rod tiers, ascending. Tier 0 is the starter (reach 1, cost 0).
/// Higher tiers reach further / bite more / stronger line.
pub static RODS: &[RodDef] = &[
    RodDef {
        tier: 0,
        name: "Old Rod",
        reach: 1,
        bite_bonus: 0,
        line_strength: 1,
        cost: 0,
    },
    RodDef {
        tier: 1,
        name: "Fiberglass Rod",
        reach: 2,
        bite_bonus: 8,
        line_strength: 2,
        cost: 500,
    },
    RodDef {
        tier: 2,
        name: "Lakecaster",
        reach: 3,
        bite_bonus: 16,
        line_strength: 3,
        cost: 2000,
    },
    RodDef {
        tier: 3,
        name: "Mastercraft Rod",
        reach: 4,
        bite_bonus: 25,
        line_strength: 5,
        cost: 8000,
