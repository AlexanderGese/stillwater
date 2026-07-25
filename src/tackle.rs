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

