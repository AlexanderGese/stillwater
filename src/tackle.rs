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
    },
];

/// The rod at a given tier (falls back to the starter if out of range).
pub fn rod(tier: u8) -> &'static RodDef {
    RODS.iter().find(|r| r.tier == tier).unwrap_or(&RODS[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rod_tier_0_is_free() {
        assert_eq!(rod(0).cost, 0);
    }

    #[test]
    fn test_rod_tier_2_is_lakecaster() {
        assert_eq!(rod(2).name, "Lakecaster");
    }

    #[test]
    fn test_rod_out_of_range_falls_back() {
        assert_eq!(rod(99).tier, 0);
        assert_eq!(rod(99).name, "Old Rod");
    }

    #[test]
    fn test_rods_strictly_increase_reach() {
        for i in 1..RODS.len() {
            assert!(RODS[i].reach > RODS[i - 1].reach);
        }
    }

    #[test]
    fn test_rods_strictly_increase_bite_bonus() {
        for i in 1..RODS.len() {
            assert!(RODS[i].bite_bonus > RODS[i - 1].bite_bonus);
        }
    }

    #[test]
    fn test_rods_strictly_increase_line_strength() {
        for i in 1..RODS.len() {
            assert!(RODS[i].line_strength > RODS[i - 1].line_strength);
        }
    }

    #[test]
    fn test_rods_strictly_increase_cost() {
        for i in 1..RODS.len() {
            assert!(RODS[i].cost > RODS[i - 1].cost);
        }
    }

    #[test]
    fn test_all_tiers_unique() {
        let mut tiers: Vec<u8> = RODS.iter().map(|r| r.tier).collect();
        tiers.sort_unstable();
        assert_eq!(tiers, vec![0, 1, 2, 3]);
    }
}
