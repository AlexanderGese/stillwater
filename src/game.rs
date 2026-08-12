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

    fn buy(&mut self, n: u8) {
        let offers = shop::offers(self.player.rod_tier, self.player.bait_id);
        let Some(off) = offers.get((n as usize).wrapping_sub(1)) else {
            return;
        };
        if off.owned {
            self.message = "You're already using that.".to_string();
            return;
        }
        if self.player.gold < off.price {
            self.message = format!("Not enough gold for {} ({}g).", off.name, off.price);
            return;
        }
        self.player.gold -= off.price;
        match off.item {
            ShopItem::Rod(tier) => {
                self.player.rod_tier = tier;
                self.message = format!("You bought the {}! Casts further now.", off.name);
            }
            ShopItem::Bait(id) => {
                self.player.bait_id = id;
                self.message = format!("You buy {} and tie it on.", off.name);
            }
        }
    }

    // ---- journal ----

    fn apply_journal(&mut self, _a: Action) {
        // Any key closes the journal.
        self.mode = Mode::Explore;
    }

    // ---- fishing ----

    fn apply_fishing(&mut self, a: Action) {
        let ctx = self.bite_ctx();
        let mut mode = std::mem::replace(&mut self.mode, Mode::Explore);
        let Mode::Fishing(ref mut s) = mode else {
            return;
        };

        if s.is_over() {
            self.finish_fishing(s);
            self.clock.advance(2);
            return; // leave mode = Explore
        }

        match &s.phase {
            fishing::Phase::Fighting(_) => {
                match a {
                    Action::Move(Dir::North) | Action::Interact => fishing::reel(s, &mut self.rng),
                    Action::Move(Dir::South) => fishing::ease(s, &mut self.rng),
                    _ => {}
                }
                self.clock.advance(1);
            }
            fishing::Phase::Bite { .. } => match a {
                Action::Interact => fishing::hook(s, &ctx),
                _ => {
                    fishing::wait_tick(s, &ctx, &mut self.rng);
                    self.clock.advance(2);
                }
            },
            _ => {
                fishing::wait_tick(s, &ctx, &mut self.rng);
                self.clock.advance(3);
            }
        }

        self.mode = mode;
        self.check_collapse();
    }

    fn finish_fishing(&mut self, s: &Session) {
        match &s.phase {
            fishing::Phase::Landed(catch) => {
                self.journal.record_catch(catch);
                let name = fish::by_id(catch.fish_id).map(|f| f.name).unwrap_or("fish");
                let value = catch.value();
                self.player.gold += value;
                self.message = format!(
                    "Landed a {} ({}cm)!  +{}g.  {}",
                    name,
                    catch.size,
                    value,
                    flavor::catch_line(&mut self.rng)
                );
                // A legendary giant gets its own story moment.
                let legendary = fish::by_id(catch.fish_id).map(|f| f.rarity >= 5).unwrap_or(false);
                if legendary && !self.legend_shown {
                    self.legend_shown = true;
                    self.show_story(crate::story::LEGEND, StoryReturn::Explore);
                }
            }
            fishing::Phase::Lost(reason) => {
                self.message = reason.to_string();
            }
            _ => {}
        }
    }

    fn bite_ctx(&self) -> fishing::BiteCtx {
        let rod = tackle::rod(self.player.rod_tier);
        let bait_bonus = bait::by_id(self.player.bait_id)
            .map(|b| b.bite_bonus)
            .unwrap_or(0);
        fishing::BiteCtx {
            season: self.calendar.season,
            tod: self.clock.tod(),
            weather: self.weather,
            bait_id: self.player.bait_id,
            bait_bonus,
            rod_bonus: rod.bite_bonus,
            line_strength: rod.line_strength,
        }
    }

    // ---- time / sleep ----

    fn check_collapse(&mut self) {
        if self.clock.should_collapse() {
            self.message = "You could barely keep your eyes open... you drift off.".to_string();
            self.overnight(true);
        }
    }

    fn sleep(&mut self) {
        self.overnight(false);
    }

    fn overnight(&mut self, passed_out: bool) {
        self.calendar.advance_day();
        self.weather = self.weather_next;
        self.weather_next = weather::roll(self.calendar.season, &mut self.rng);
        self.clock.reset_morning();
        self.player.energy = if passed_out { MAX_ENERGY / 2 } else { MAX_ENERGY };
        self.mode = Mode::Explore;
        let greet = flavor::greeting(&mut self.rng);
        let mood = flavor::weather_line(self.weather, &mut self.rng);
        self.message = if passed_out {
            format!("You slept where you dropped. {} {}", greet, mood)
        } else {
            format!("{} {}", greet, mood)
        };
    }
}

/// The water type a tile represents, if any. An area with a fixed `water_kind`
/// (river/marsh) types all its water that way; otherwise it's per-tile.
pub fn water_type(t: Tile, area_water: Option<WaterType>) -> Option<WaterType> {
    if !t.is_water() {
        return None;
    }
    match area_water {
        Some(k) => Some(k),
        None => match t {
            Tile::DeepWater => Some(WaterType::Deep),
            _ => Some(WaterType::Shallow),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Dir;

    #[test]
    fn player_only_ever_stands_on_walkable_tiles() {
        let mut g = Game::new();
        assert!(g.world.map().walkable(g.player.pos));
        let dirs = [Dir::North, Dir::East, Dir::South, Dir::West];
        for i in 0..120 {
            g.apply(Action::Move(dirs[i % 4]));
            assert!(
                g.world.map().walkable(g.player.pos),
                "stood on a non-walkable tile after moving"
            );
        }
    }

    #[test]
    fn quit_stops_running() {
        let mut g = Game::new();
        g.apply(Action::Quit);
        assert!(!g.running);
    }

    #[test]
    fn sleeping_at_the_bed_advances_day_and_restores_energy() {
        use crate::geom::Point;
        use crate::tile::Tile;
        let mut g = Game::new();
        // Find the bed in the homestead and stand next to it, facing it.
        let mut placed = false;
        let map = g.world.map();
        'outer: for y in 0..map.h {
            for x in 0..map.w {
                if map.get(Point::new(x, y)) == Tile::Bed {
                    for (dir, off) in [
                        (Dir::South, (0, -1)),
                        (Dir::North, (0, 1)),
                        (Dir::East, (-1, 0)),
                        (Dir::West, (1, 0)),
                    ] {
                        let sp = Point::new(x + off.0, y + off.1);
                        if map.walkable(sp) {
                            g.player.pos = sp;
                            g.player.facing = dir;
                            placed = true;
                            break 'outer;
                        }
                    }
                }
            }
        }
        assert!(placed, "expected a walkable tile next to a bed");
        g.player.energy = 5;
        let day0 = g.calendar.day;
        g.apply(Action::Interact);
        assert_eq!(g.calendar.day, day0 + 1);
        assert_eq!(g.player.energy, MAX_ENERGY);
    }

    #[test]
    fn staying_up_past_two_am_collapses_into_next_day() {
        let mut g = Game::new();
        let day0 = g.calendar.day;
        for _ in 0..200 {
            g.apply(Action::Wait);
        }
        assert_eq!(g.calendar.day, day0 + 1);
        assert_eq!(g.player.energy, MAX_ENERGY / 2);
    }

    #[test]
    fn buying_a_rod_spends_gold_and_upgrades() {
        let mut g = Game::new();
        g.mode = Mode::Shop;
        g.player.gold = 1000;
        // Offer 1 is the next rod (Fiberglass, 500g).
        g.apply(Action::Buy(1));
        assert_eq!(g.player.rod_tier, 1);
        assert_eq!(g.player.gold, 500);
    }

    #[test]
    fn buying_without_gold_is_refused() {
        let mut g = Game::new();
        g.mode = Mode::Shop;
        g.player.gold = 10;
        g.apply(Action::Buy(1)); // 500g rod
        assert_eq!(g.player.rod_tier, 0);
        assert_eq!(g.player.gold, 10);
        assert!(g.message.contains("Not enough"));
    }

    #[test]
    fn landing_a_fish_records_it_in_the_journal() {
        let mut g = Game::with_seed(7);
        g.mode = Mode::Fishing(Session::new(WaterType::Shallow));
        for _ in 0..300 {
            match &g.mode {
                Mode::Fishing(s) => match &s.phase {
                    fishing::Phase::Waiting { .. } => g.apply(Action::Wait),
                    fishing::Phase::Bite { .. } => g.apply(Action::Interact),
                    fishing::Phase::Fighting(f) => {
                        if f.darting {
                            g.apply(Action::Move(Dir::South))
                        } else {
                            g.apply(Action::Interact)
                        }
                    }
                    _ => {
                        g.apply(Action::Interact);
                        break;
                    }
                },
                _ => break,
            }
