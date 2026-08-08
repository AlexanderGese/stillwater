//! Onboarding: a paged intro shown on a new game, plus a context-aware hint
//! line that tells the player exactly what to do next based on the game state.

use crate::fishing::Phase;
use crate::game::{Game, Mode};
use crate::world;

/// The how-to-play pages, shown after the opening story (and from the menu).
pub static HOWTO: &[&str] = &[
    "\
        G E T T I N G   A R O U N D

   Walk with  W A S D  (or the arrow keys).

   Each place is one screen; walk into an OPEN EDGE of the map to
   travel to the next area of the valley.

                                   [space] next",
    "\
        H O W   T O   F I S H

   Stand at the water's edge, FACE the water, and press  E  to cast.

   Wait for a bite (the line will twitch), then press  E  again the
   moment it bites to set the hook.

                                   [space] next",
    "\
        L A N D I N G   T H E   C A T C H

   Once a fish is on, watch the two bars:

        W  =  REEL it in   (fills the catch bar)
        S  =  EASE off     (bleeds the tension)

   Reel when the line is steady \u{2014} but EASE the moment the fish
   DARTS, or the line will snap and it gets away.

                                   [space] next",
    "\
        G R O W I N G

   Landed fish earn gold and fill your JOURNAL (press  J ).

   Spend gold at the SHOP  ( H )  on better rods and bait.
   Sleep in your BED  ( B )  to pass into the next day \u{2014} the
   seasons and weather change what's biting.

                                   [space] next",
    "\
        R E S T O R I N G   T H E   V A L L E Y

   Read the NOTICE BOARD  ( N )  in town to fund restoration
   projects. Each one opens a new stretch of water \u{2014} the marsh,
   the river, and the deep lake \u{2014} with new fish to discover.

   Press  ?  any time to see the controls.

                                 [space] B E G I N",
];

/// Number of steps in the guided (step-by-step) tutorial.
pub const GUIDE_STEPS: usize = 4;

/// The banner shown for the current guided-tutorial step.
pub fn guide_prompt(step: usize) -> &'static str {
    match step {
        0 => "STEP 1 of 4:  walk to the water's edge and face the water (the ~ tiles)",
        1 => "STEP 2 of 4:  press [e] to cast your line into the water",
        2 => "STEP 3 of 4:  wait for a bite, [e] to hook, then [w]/[s] to land your first fish!",
        _ => "STEP 4 of 4:  sleep at your bed (stand by the B and press [e]) to start a new day",
    }
}

/// Has the player satisfied the given guided-tutorial step?
pub fn guide_done(step: usize, g: &Game) -> bool {
    match step {
        0 => g.faces_water(),
        1 => matches!(g.mode, Mode::Fishing(_)),
        2 => g.journal.caught_total() >= 1,
        _ => g.calendar.day >= 2,
    }
}

/// A short, situation-specific hint for the current game state, or None.
pub fn hint(g: &Game) -> Option<&'static str> {
    match &g.mode {
        Mode::Fishing(s) => Some(match &s.phase {
            Phase::Waiting { .. } => "hold still and wait for a bite... (press any key to wait)",
            Phase::Bite { .. } => "a bite! press [e] RIGHT NOW to set the hook!",
            Phase::Fighting(f) => {
                if f.darting {
                    "it's thrashing! press [s] to EASE off before the line snaps!"
                } else {
                    "line's steady \u{2014} press [w] to REEL it in!"
                }
            }
            Phase::Landed(_) | Phase::Lost(_) => "press [e] to carry on",
        }),
        Mode::Explore => explore_hint(g),
        _ => None,
    }
}

fn explore_hint(g: &Game) -> Option<&'static str> {
    // Guide the very first catch.
    if g.journal.caught_total() == 0 {
        if g.faces_water() {
            return Some("you're facing the water \u{2014} press [e] to cast your line!");
        }
        return Some("walk to the water (the ~ tiles), face it, and press [e] to fish");
    }
    // Tired? Point them to bed.
    if g.player.energy <= 20 {
        return Some("running low on energy \u{2014} sleep at your bed [B] to start a fresh day");
    }
    // Enough gold for the first upgrade.
    if g.player.rod_tier == 0 && g.player.gold >= 500 {
        return Some("you've saved enough \u{2014} the shop [H] has a rod that casts farther");
    }
    // In town: nudge toward restoration.
    if g.world.current() == world::TOWN {
        return Some("read the notice board [N] to fund restoration and open new waters");
    }
    // In a freshly-opened wild area, encourage discovery.
    if matches!(
        g.world.current(),
        world::RIVER | world::MARSH | world::DEEPLAKE
    ) {
        return Some("new waters hold new fish \u{2014} cast around and check your journal [j]");
    }
    // Once established, nudge toward better bait.
    if g.player.bait_id == 1 && g.player.gold >= 200 && g.journal.caught_total() >= 3 {
        return Some("better bait at the shop [H] tempts rarer fish onto the hook");
    }
    // Gentle steady-state reminder.
    if g.faces_water() {
        return Some("press [e] to cast");
    }
    Some("catch fish for gold, upgrade at the shop [H], open new areas at the town board [N]")
}
