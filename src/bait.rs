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

    #[test]
    fn test_ids_unique_and_sequential() {
        let ids: Vec<u16> = BAIT.iter().map(|b| b.id).collect();
        // Check all ids are present from 1 to 6
        assert_eq!(ids.len(), 6);
        for expected_id in 1..=6 {
            assert!(ids.contains(&expected_id), "Missing id: {}", expected_id);
        }
        // Check no duplicates
        for (i, id1) in ids.iter().enumerate() {
            for id2 in ids.iter().skip(i + 1) {
                assert_ne!(id1, id2, "Duplicate id found");
            }
        }
    }

    #[test]
    fn test_by_id_cricket() {
        let cricket = by_id(3);
        assert!(cricket.is_some());
        let bait = cricket.unwrap();
        assert_eq!(bait.name, "Cricket");
        assert_eq!(bait.id, 3);
        assert_eq!(bait.cost, 20);
        assert_eq!(bait.bite_bonus, 15);
    }

    #[test]
    fn test_by_id_none() {
        assert!(by_id(99).is_none());
    }

    #[test]
    fn test_costs_strictly_ascending() {
        for i in 0..BAIT.len() - 1 {
            assert!(
                BAIT[i].cost < BAIT[i + 1].cost,
                "Costs not strictly ascending: {} >= {}",
                BAIT[i].cost,
                BAIT[i + 1].cost
            );
        }
    }

    #[test]
    fn test_bite_bonus_strictly_ascending() {
        for i in 0..BAIT.len() - 1 {
            assert!(
                BAIT[i].bite_bonus < BAIT[i + 1].bite_bonus,
                "Bite bonus not strictly ascending: {} >= {}",
                BAIT[i].bite_bonus,
                BAIT[i + 1].bite_bonus
            );
        }
    }
}
