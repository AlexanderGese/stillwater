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
        id: 4,
        name: "Shrimp",
        cost: 35,
        bite_bonus: 22,
    },
    BaitDef {
        id: 5,
        name: "Roe",
        cost: 55,
        bite_bonus: 30,
    },
    BaitDef {
        id: 6,
        name: "Glowbait",
        cost: 90,
        bite_bonus: 40,
    },
];

pub fn by_id(id: u16) -> Option<&'static BaitDef> {
    BAIT.iter().find(|b| b.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

