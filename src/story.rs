//! The story of Stillwater Farm.
//! Each `&str` is ONE screen/page of story text (roughly 8-12 short lines that
//! fit an 80-column terminal), the last line usually a `[space] ...` prompt.
//! Keep the public names/signatures; the game pages through these.

/// The opening, shown when you begin a new game (before the how-to pages).
/// Several unhurried, cozy pages that set up who you are and why you've come.
pub static OPENING: &[&str] = &[
    "\
        S T I L L W A T E R   F A R M

   The letter finds you on a grey Tuesday, forwarded
   twice, corners soft from travel.

   Great-Aunt Wren has left you the cottage on the lake.
   Nobody in the family had spoken to her in years, but
   she remembered you. She always did.

   The lawyer's note is short. A key, a deed, a valley
   you have never seen. You pack one bag and go.

                                   [space] next",
    "\
   The bus lets you off where the road gives up.

   From there it's a walk, downhill, through birch and
   old stone fences gone soft with moss. The trees open
   all at once and there it is below you -

   Stillwater. Flat and grey as a held breath, ringed
   in reeds gone wild, a thin thread of river losing
   itself in silted mud. No boats. No birds calling.
   Just the hush of a place that has been waiting.

