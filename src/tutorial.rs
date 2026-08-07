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

