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

pub enum Mode {
    Menu { sel: usize },     // main menu
    Settings { sel: usize }, // options screen
    Story { page: usize },   // paged story/intro text
    Explore,
    Fishing(Session),
    Shop,
    Journal,
    Restore,
    Help,
}

/// Where to go when the current story sequence finishes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StoryReturn {
    Howto,   // opening -> the how-to-play pages
    Play,    // how-to -> the world (and start the guided tutorial)
    Explore, // into the world
    Menu,    // back to the main menu
    Ending,  // this beat -> the ending -> the world
}

pub struct Game {
    pub world: World,
    pub player: Player,
    pub rng: Rng,
    pub calendar: Calendar,
    pub clock: Clock,
    pub weather: Weather,
    pub weather_next: Weather,
    pub journal: Journal,
    pub mode: Mode,
    pub running: bool,
    pub message: String,
    pub seed: u64,
    pub loaded_from_save: bool,
    pub story_pages: &'static [&'static str],
    pub story_next: StoryReturn,
    pub ending_shown: bool,
    pub legend_shown: bool,
    pub settings: Settings,
    pub guide_step: Option<usize>, // step-by-step tutorial progress
}

impl Game {
    pub fn new() -> Game {
        Game::with_seed(1)
    }

