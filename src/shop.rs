//! Shop offerings: buy the next rod tier and any bait. CONTRACT STUB — an
//! implementer agent fills `offers` + tests. Keep the public types/signatures
//! (game.rs drives purchases and render.rs lists these).

use crate::bait;
use crate::tackle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShopItem {
    Rod(u8),   // rod tier to purchase
    Bait(u16), // bait id to buy/equip
}

#[derive(Clone, Debug)]
pub struct Offer {
    pub item: ShopItem,
    pub name: &'static str,
    pub price: u32,
    pub owned: bool, // rod already owned, or bait currently equipped
}

/// The shop's current offerings given what the player already has.
/// AGENT: implement — list the NEXT rod tier above `current_rod_tier` (only the
/// next one, if any exists in `tackle::RODS`), then every bait in `bait::BAIT`.
/// Mark the equipped bait / owned rods with `owned = true`. Prices come from
/// `RodDef.cost` / `BaitDef.cost`.
pub fn offers(current_rod_tier: u8, current_bait_id: u16) -> Vec<Offer> {
    let mut result = Vec::new();

    // Add the next rod tier if it exists
    let next_rod_tier = current_rod_tier + 1;
    if let Some(next_rod) = tackle::RODS.iter().find(|r| r.tier == next_rod_tier) {
        result.push(Offer {
            item: ShopItem::Rod(next_rod.tier),
            name: next_rod.name,
            price: next_rod.cost,
            owned: false,
        });
    }

    // Add all baits
    for bait_def in bait::BAIT.iter() {
        result.push(Offer {
            item: ShopItem::Bait(bait_def.id),
            name: bait_def.name,
            price: bait_def.cost,
            owned: bait_def.id == current_bait_id,
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_0_offers_fiberglass_rod_as_first() {
        let offers_list = offers(0, 1);
        assert!(!offers_list.is_empty());
        let first = &offers_list[0];
        assert_eq!(first.name, "Fiberglass Rod");
        assert_eq!(first.price, 500);
        assert_eq!(first.item, ShopItem::Rod(1));
        assert!(!first.owned);
    }

    #[test]
    fn test_tier_0_has_one_rod_plus_all_baits() {
        let offers_list = offers(0, 1);
        let expected_count = 1 + bait::BAIT.len();
        assert_eq!(offers_list.len(), expected_count);
    }

    #[test]
    fn test_top_rod_tier_no_rod_offer() {
        let max_tier = tackle::RODS.iter().map(|r| r.tier).max().unwrap();
        let offers_list = offers(max_tier, 1);
        // Should only have baits, no rod offers
        assert_eq!(offers_list.len(), bait::BAIT.len());
        for offer in offers_list {
            assert!(matches!(offer.item, ShopItem::Bait(_)));
        }
    }

