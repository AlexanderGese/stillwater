use crate::bait;
use crate::calendar::Calendar;
use crate::clock::Clock;
use crate::fish;
use crate::fishing::{self, Session};
use crate::flavor;
use crate::geom::Dir;
use crate::journal::Journal;
use crate::player::{Player, MAX_ENERGY};
use crate::restore;
use crate::rng::Rng;
use crate::season::WaterType;
use crate::settings::Settings;
use crate::shop::{self, ShopItem};
use crate::tackle;
use crate::tile::Tile;
use crate::weather::{self, Weather};
use crate::world::World;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    Move(Dir),
    Interact, // cast at water / hook / reel / sleep at bed / open shop / continue
    OpenJournal,
    OpenHelp,
    Buy(u8), // shop: buy the nth listed offer (1-based)
    Back,    // close a menu
    Wait,
    Any, // any other key (advances the intro / closes help)
    Quit,
}

