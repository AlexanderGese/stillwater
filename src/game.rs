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

    pub fn with_seed(seed: u64) -> Game {
        let world = World::new();
        let start = world.area().start;
        let calendar = Calendar::new();
        let mut rng = Rng::new(seed);
        let weather = weather::roll(calendar.season, &mut rng);
        let weather_next = weather::roll(calendar.season, &mut rng);
        let mut player = Player::new(start);
        player.bait_id = 1; // start with a worm on the hook
        Game {
            world,
            player,
            rng,
            calendar,
            clock: Clock::new(),
            weather,
            weather_next,
            journal: Journal::new(),
            mode: Mode::Explore,
            running: true,
            message: String::new(),
            seed,
            loaded_from_save: false,
            story_pages: &[],
            story_next: StoryReturn::Explore,
            ending_shown: false,
            legend_shown: false,
            settings: Settings::new(),
            guide_step: None,
        }
    }

    /// Enter the main menu (used at launch).
    pub fn to_menu(&mut self) {
        self.mode = Mode::Menu { sel: 0 };
    }

    /// Reset to a brand-new game and roll into the opening story (keeping the
    /// player's settings).
    fn start_new(&mut self) {
        let seed = self.seed.wrapping_add(0x9E3779B9);
        let settings = self.settings;
        *self = Game::with_seed(seed);
        self.settings = settings;
        self.show_story(crate::story::OPENING, StoryReturn::Howto);
    }

    fn show_story(&mut self, pages: &'static [&'static str], next: StoryReturn) {
        self.story_pages = pages;
        self.story_next = next;
        self.mode = Mode::Story { page: 0 };
    }

    pub fn apply(&mut self, a: Action) {
        if a == Action::Quit {
            self.running = false;
            return;
        }
        match self.mode {
            Mode::Menu { .. } => self.apply_menu(a),
            Mode::Settings { .. } => self.apply_settings(a),
            Mode::Story { .. } => self.apply_story(a),
            Mode::Explore => self.apply_explore(a),
            Mode::Fishing(_) => self.apply_fishing(a),
            Mode::Shop => self.apply_shop(a),
            Mode::Journal => self.apply_journal(a),
            Mode::Restore => self.apply_restore(a),
            Mode::Help => self.mode = Mode::Explore, // any key closes help
        }
        self.tick_guide();
    }

    /// Advance the step-by-step tutorial when the current step is satisfied.
    fn tick_guide(&mut self) {
        if let Some(s) = self.guide_step {
            if crate::tutorial::guide_done(s, self) {
                if s + 1 >= crate::tutorial::GUIDE_STEPS {
                    self.guide_step = None;
                    self.message =
                        "Tutorial complete! You've got it. Enjoy your days at Stillwater.".to_string();
                } else {
                    self.guide_step = Some(s + 1);
                }
            }
        }
    }

    fn apply_settings(&mut self, a: Action) {
        let n = Settings::count();
        let sel = if let Mode::Settings { sel } = self.mode {
            sel
        } else {
            0
        };
        match a {
            Action::Move(Dir::North) => self.mode = Mode::Settings { sel: (sel + n - 1) % n },
            Action::Move(Dir::South) => self.mode = Mode::Settings { sel: (sel + 1) % n },
            Action::Interact => self.settings.toggle(sel),
            Action::Buy(k) => {
                let i = (k as usize).wrapping_sub(1);
                if i < n {
                    self.settings.toggle(i);
                }
            }
            Action::Back => self.mode = Mode::Menu { sel: 0 },
            _ => {}
        }
    }

    pub fn menu_options(&self) -> &'static [&'static str] {
        if self.loaded_from_save {
            &["Continue", "New Game", "How to Play", "Settings", "Quit"]
        } else {
            &["New Game", "How to Play", "Settings", "Quit"]
        }
    }

    fn apply_menu(&mut self, a: Action) {
        let opts = self.menu_options();
        let n = opts.len();
        let sel = if let Mode::Menu { sel } = self.mode { sel } else { 0 };
        match a {
            Action::Move(Dir::North) => self.mode = Mode::Menu { sel: (sel + n - 1) % n },
            Action::Move(Dir::South) => self.mode = Mode::Menu { sel: (sel + 1) % n },
            Action::Buy(k) => {
                let i = (k as usize).wrapping_sub(1);
                if i < n {
                    self.activate_menu(opts[i]);
                }
            }
            Action::Interact => self.activate_menu(opts[sel]),
            _ => {}
        }
    }

    fn activate_menu(&mut self, opt: &str) {
        match opt {
            "Continue" => self.mode = Mode::Explore,
            "New Game" => self.start_new(),
            "How to Play" => self.show_story(crate::tutorial::HOWTO, StoryReturn::Menu),
            "Settings" => self.mode = Mode::Settings { sel: 0 },
            "Quit" => self.running = false,
            _ => {}
        }
    }

    fn apply_story(&mut self, a: Action) {
        let page = if let Mode::Story { page } = self.mode { page } else { 0 };
        let last = self.story_pages.len().saturating_sub(1);
        let advance = matches!(a, Action::Back) || page >= last;
        if advance {
            match self.story_next {
                StoryReturn::Howto => self.show_story(crate::tutorial::HOWTO, StoryReturn::Play),
                StoryReturn::Play => {
                    self.mode = Mode::Explore;
                    if self.settings.guide {
                        self.guide_step = Some(0);
                    }
                    self.message = "Your days at Stillwater begin.".to_string();
                }
                StoryReturn::Explore => {
                    self.mode = Mode::Explore;
                    self.message = "Your days at Stillwater begin.".to_string();
                }
                StoryReturn::Menu => self.mode = Mode::Menu { sel: 0 },
                StoryReturn::Ending => {
                    self.show_story(crate::story::ENDING, StoryReturn::Explore)
                }
            }
        } else {
            self.mode = Mode::Story { page: page + 1 };
        }
    }

    /// Does the player currently face a castable stretch of water within reach?
    pub fn faces_water(&self) -> bool {
        let reach = tackle::rod(self.player.rod_tier).reach;
        let mut p = self.player.pos;
        for _ in 0..reach {
            p = p.step(self.player.facing);
            let t = self.world.map().get(p);
            if t.is_water() {
                return true;
            }
            if t == Tile::Wall {
                break;
            }
        }
        false
    }

    // ---- explore ----

    fn apply_explore(&mut self, a: Action) {
        match a {
            Action::Move(d) => {
                self.player.facing = d;
                let target = self.player.pos.step(d);
                if self.world.map().walkable(target) {
                    self.player.pos = target;
                    self.message.clear();
                    self.clock.advance(2);
                    self.check_collapse();
                } else if !self.world.map().in_bounds(target) {
                    self.try_transition(d);
                }
            }
            Action::Interact => self.interact(),
            Action::OpenJournal => self.mode = Mode::Journal,
            Action::OpenHelp => self.mode = Mode::Help,
            Action::Wait => {
                self.clock.advance(10);
                self.check_collapse();
            }
            Action::Back => {
                if self.guide_step.take().is_some() {
                    self.message = "Tutorial skipped. Press [?] any time for help.".to_string();
                }
            }
            _ => {}
        }
    }

    fn interact(&mut self) {
        let here = self.world.map().get(self.player.pos);
        let facing = self.world.map().get(self.player.pos.step(self.player.facing));
        if here == Tile::Bed || facing == Tile::Bed {
            self.sleep();
        } else if here == Tile::Shop || facing == Tile::Shop {
            if self.clock.is_past(Clock::SHOP_CLOSE / 60) {
                self.message = "The tackle shop is shuttered for the night.".to_string();
            } else {
                self.mode = Mode::Shop;
                self.message = "Welcome! Buy a rod or some bait.".to_string();
            }
        } else if here == Tile::Sign || facing == Tile::Sign {
            self.mode = Mode::Restore;
            self.message = "You read the restoration notice board.".to_string();
        } else {
            self.try_cast();
        }
    }

    fn try_transition(&mut self, d: Dir) {
        let Some(res) = self.world.exit_toward(d) else {
            return;
        };
        match res {
            Ok((to, entry)) => {
                self.world.set_current(to);
                self.player.pos = entry;
                self.player.facing = d;
                self.clock.advance(5);
                self.message = format!("You make your way to {}.", self.world.area_name());
                self.check_collapse();
            }
            Err(name) => {
                self.message = format!("The way is blocked \u{2014} you need to \"{}\" first.", name);
            }
        }
    }

    // ---- restoration ----

    fn apply_restore(&mut self, a: Action) {
        match a {
            Action::Buy(n) => self.fund_project(n),
            Action::Back | Action::Interact => {
                self.mode = Mode::Explore;
                self.message = "You step back from the board.".to_string();
            }
            _ => {}
        }
    }

    fn fund_project(&mut self, n: u8) {
        let i = (n as usize).wrapping_sub(1);
        let Some(p) = restore::project(i) else {
            return;
        };
        if self.world.is_funded(i) {
            self.message = format!("\"{}\" is already done.", p.name);
            return;
        }
        if self.player.gold < p.cost {
            self.message = format!("You need {}g to fund \"{}\".", p.cost, p.name);
            return;
        }
        self.player.gold -= p.cost;
        self.world.fund(i);
        self.message = format!("Funded \"{}\"!", p.name);
        // Play the story beat for this project; if it's the last one, roll the
        // beat into the ending.
        let all_done = (0..restore::PROJECTS.len()).all(|k| self.world.is_funded(k));
        if all_done && !self.ending_shown {
            self.ending_shown = true;
            self.show_story(crate::story::beat(i), StoryReturn::Ending);
        } else {
            self.show_story(crate::story::beat(i), StoryReturn::Explore);
        }
    }

    fn try_cast(&mut self) {
        let reach = tackle::rod(self.player.rod_tier).reach;
        let wk = self.world.water_kind();
        let mut p = self.player.pos;
        let mut found = None;
        for _ in 0..reach {
            p = p.step(self.player.facing);
            let t = self.world.map().get(p);
            if let Some(w) = water_type(t, wk) {
                found = Some(w);
                break;
            }
            if t == Tile::Wall {
                break;
            }
        }
        match found {
            Some(w) => {
                if self.player.energy <= 0 {
                    self.message = "You're too worn out to cast. Get some sleep.".to_string();
                    return;
                }
                self.player.energy -= 3;
                self.clock.advance(10);
                self.mode = Mode::Fishing(Session::new(w));
                self.message = "You cast your line out over the water...".to_string();
                self.check_collapse();
            }
            None => {
                self.message = "No water within reach. Face the lake and try again.".to_string();
            }
        }
    }

    // ---- shop ----

    fn apply_shop(&mut self, a: Action) {
        match a {
            Action::Buy(n) => self.buy(n),
            Action::Back | Action::Interact => {
                self.mode = Mode::Explore;
                self.message = "You step away from the counter.".to_string();
            }
            _ => {}
        }
    }

