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

