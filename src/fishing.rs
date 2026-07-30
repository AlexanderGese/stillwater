//! The fishing loop: cast -> wait for a bite -> hook -> reel fight -> land.
//! Pure and deterministic given an `Rng`, so the whole thing is testable headless.

use crate::fish::{self, Catch, FishDef};
use crate::rng::Rng;
use crate::season::{Season, TimeOfDay, WaterType};
use crate::weather::Weather;

/// Everything the bite/fight math needs from the wider game.
#[derive(Clone, Copy)]
pub struct BiteCtx {
    pub season: Season,
    pub tod: TimeOfDay,
    pub weather: Weather,
    pub bait_id: u16,
    pub bait_bonus: i32,
    pub rod_bonus: i32,
    pub line_strength: u8,
}

/// The active reel fight against a hooked fish.
pub struct Fight {
    pub fish_id: u16,
    pub progress: i32, // 0..=100 -> lands (or clears a boss phase)
    pub slack: i32,    // 0..=100 -> line snaps
    pub darting: bool, // fish is pulling hard right now (reel = lots of slack)
    pub difficulty: u8,
    pub line_strength: u8,
    pub boss: bool,       // a legendary giant: a multi-phase fight
    pub phases_left: u8,  // boss phases remaining (1 = final)
    pub surge: bool,      // just cleared a phase — the giant surges
}

pub enum Phase {
    Waiting { waited: i32 },
    Bite { fish_id: u16, patience: i32 },
    Fighting(Fight),
    Landed(Catch),
    Lost(&'static str),
}

pub struct Session {
    pub water: WaterType,
    pub phase: Phase,
}

impl Session {
    pub fn new(water: WaterType) -> Session {
        Session {
            water,
            phase: Phase::Waiting { waited: 0 },
        }
    }

    pub fn is_over(&self) -> bool {
        matches!(self.phase, Phase::Landed(_) | Phase::Lost(_))
    }
}

const BITE_BASE: i32 = 16; // base % bite chance on the first wait tick
const MAX_WAIT: i32 = 8; // give up after this many ticks with no bite

/// Advance one tick while waiting for a bite (or waiting to hook a nibble).
pub fn wait_tick(s: &mut Session, ctx: &BiteCtx, rng: &mut Rng) {
    match &mut s.phase {
        Phase::Waiting { waited } => {
            let avail = fish::available(ctx.season, s.water, ctx.tod);
            if avail.is_empty() {
                s.phase = Phase::Lost("Nothing lives in these waters right now.");
                return;
            }
            let w = *waited;
            let chance = (BITE_BASE + w * 5 + ctx.weather.bite_bonus() + ctx.bait_bonus + ctx.rod_bonus)
                .clamp(2, 95);
            if (rng.below(100) as i32) < chance {
                let f = pick_fish(&avail, ctx.bait_id, rng);
                s.phase = Phase::Bite {
                    fish_id: f.id,
                    patience: 3,
                };
            } else if w >= MAX_WAIT {
                s.phase = Phase::Lost("Not even a nibble. You reel the line back in.");
            } else {
                *waited = w + 1;
            }
        }
        Phase::Bite { patience, .. } => {
            // Dawdle too long and the fish spits the hook.
            if *patience <= 1 {
                s.phase = Phase::Lost("The fish spat out the bait and slipped away.");
            } else {
                *patience -= 1;
            }
        }
        _ => {}
    }
}

fn pick_fish<'a>(avail: &[&'a FishDef], bait_id: u16, rng: &mut Rng) -> &'a FishDef {
    // Common fish (low rarity) are far likelier; matching bait doubles the odds.
    let weights: Vec<u32> = avail
        .iter()
        .map(|f| {
            let base = (6 - f.rarity.min(5)) as u32;
            let base = base * base;
            if f.bait_pref != 0 && f.bait_pref == bait_id {
                base * 2
            } else {
                base
            }
        })
        .collect();
    let total: u32 = weights.iter().sum::<u32>().max(1);
    let mut r = rng.below(total);
    for (i, w) in weights.iter().enumerate() {
        if r < *w {
            return avail[i];
        }
        r -= *w;
    }
    avail[avail.len() - 1]
}

/// Set the hook: a biting fish becomes a fight.
pub fn hook(s: &mut Session, ctx: &BiteCtx) {
    if let Phase::Bite { fish_id, .. } = s.phase {
        let def = fish::by_id(fish_id);
        let difficulty = def.map(|f| f.difficulty).unwrap_or(1);
        let boss = def.map(|f| f.rarity).unwrap_or(1) >= 5;
        s.phase = Phase::Fighting(Fight {
            fish_id,
            progress: 0,
            slack: 0,
            darting: false,
            difficulty,
            line_strength: ctx.line_strength,
            boss,
            phases_left: if boss { 3 } else { 1 },
            surge: false,
        });
    }
}

/// Is the current fish a legendary (fought as a boss)?
pub fn is_boss(s: &Session) -> bool {
    matches!(&s.phase, Phase::Fighting(f) if f.boss)
}

/// Reel hard: gains progress, but adds a lot of slack while the fish is darting.
pub fn reel(s: &mut Session, rng: &mut Rng) {
    if let Phase::Fighting(f) = &mut s.phase {
        f.progress += 12;
        let pull = if f.darting {
            (24 - f.line_strength as i32 * 2).max(8)
        } else {
            4
        };
        f.slack += pull;
        step_fight(f, rng);
        resolve(s, rng);
    }
}

/// Ease off: bleeds slack; makes a little progress only while the fish is calm.
pub fn ease(s: &mut Session, rng: &mut Rng) {
    if let Phase::Fighting(f) = &mut s.phase {
        f.slack = (f.slack - 16).max(0);
        f.progress += if f.darting { 0 } else { 3 };
        step_fight(f, rng);
        resolve(s, rng);
    }
}

fn step_fight(f: &mut Fight, rng: &mut Rng) {
    // Harder fish (and bosses) dart more often, tensing the reel-vs-ease read.
    let dart_chance = 20 + f.difficulty as u32 * 5 + if f.boss { 15 } else { 0 };
    f.darting = (rng.below(100)) < dart_chance;
}

enum Outcome {
    Continue,
    Snap,
    Surge,
    Land(u16),
}

fn resolve(s: &mut Session, rng: &mut Rng) {
    let out = if let Phase::Fighting(f) = &mut s.phase {
        f.surge = false;
        if f.slack >= 100 {
            Outcome::Snap
        } else if f.progress >= 100 {
            if f.phases_left > 1 {
                // Cleared a boss phase: it dives and surges back harder.
                f.phases_left -= 1;
                f.progress = 30;
                f.slack = (f.slack + 12).min(85);
                f.darting = true;
                f.surge = true;
                Outcome::Surge
            } else {
                Outcome::Land(f.fish_id)
            }
        } else {
            Outcome::Continue
        }
    } else {
        Outcome::Continue
    };
    match out {
        Outcome::Snap => s.phase = Phase::Lost("The line snapped! It's gone."),
        Outcome::Land(id) => {
            let catch = roll_catch(id, rng);
            s.phase = Phase::Landed(catch);
        }
        Outcome::Surge | Outcome::Continue => {}
    }
}

fn roll_catch(fish_id: u16, rng: &mut Rng) -> Catch {
    let (lo, hi) = fish::by_id(fish_id)
        .map(|f| (f.size_min, f.size_max))
        .unwrap_or((1, 1));
    let span = (hi.saturating_sub(lo)) as u32 + 1;
    let size = lo + rng.below(span) as u16;
    Catch { fish_id, size }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> BiteCtx {
        BiteCtx {
            season: Season::Spring,
            tod: TimeOfDay::Morning,
            weather: Weather::Rain,
            bait_id: 1,
            bait_bonus: 5,
            rod_bonus: 0,
            line_strength: 3,
        }
    }

    #[test]
    fn waiting_eventually_bites_in_stocked_water() {
        let mut rng = Rng::new(42);
        let mut s = Session::new(WaterType::Shallow);
        let c = ctx();
        let mut got_bite = false;
        for _ in 0..12 {
            wait_tick(&mut s, &c, &mut rng);
            if matches!(s.phase, Phase::Bite { .. }) {
                got_bite = true;
                break;
            }
            if s.is_over() {
                break;
            }
        }
        assert!(got_bite, "should hook a bite in stocked shallow spring water");
    }

    #[test]
    fn careful_angler_lands_the_fish() {
        let mut rng = Rng::new(7);
        let mut s = Session::new(WaterType::Shallow);
        let c = ctx();
        // Get to a bite.
        for _ in 0..12 {
            wait_tick(&mut s, &c, &mut rng);
            if matches!(s.phase, Phase::Bite { .. }) {
                break;
            }
        }
        assert!(matches!(s.phase, Phase::Bite { .. }));
        hook(&mut s, &c);
        // Play it smart: ease when the fish darts, reel when it's calm.
        for _ in 0..80 {
            if s.is_over() {
                break;
            }
            let darting = matches!(&s.phase, Phase::Fighting(f) if f.darting);
            if darting {
                ease(&mut s, &mut rng);
            } else {
                reel(&mut s, &mut rng);
            }
        }
        match &s.phase {
            Phase::Landed(catch) => {
                let def = fish::by_id(catch.fish_id).unwrap();
                assert!(catch.size >= def.size_min && catch.size <= def.size_max);
                assert!(catch.value() > 0);
            }
            other => panic!("expected to land the fish, got {:?}", phase_name(other)),
        }
