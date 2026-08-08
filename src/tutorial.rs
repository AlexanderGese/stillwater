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

