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

                                   [space] next",
    "\
   You stop halfway down the path, because your feet
   remember it before you do.

   You were nine the last summer you came here, feet
   bare and stung with nettles, Wren teaching you to tie
   a knot with your tongue between your teeth in
   concentration. Six weeks of jam jars full of fireflies,
   thunderstorms watched from the porch wrapped in one
   blanket, more fish caught by her than you but she
   always said the numbers were close.

   You didn't know it was the last summer. Nobody tells
   you that in advance.

                                   [space] next",
    "\
   You never knew exactly what happened between her and
   your grandfather, only that it happened quietly, over
   years, the way a bank erodes instead of breaks.

   A disagreement about the old farm, maybe, or just two
   stubborn people who stopped reaching for the phone
   first. Christmas cards got shorter. Then they stopped.
   Wren stayed here anyway, alone with the lake, and if
   it was lonely she never once said so in the letters
   you did get.

   You think now she wasn't hiding from the family. She
   was just keeping something the rest of you had
   forgotten how to want.

                                   [space] next",
    "\
   The cottage is smaller than you pictured and leans
   a little, like it's listening for something.

   Inside: a cold stove, a chair by the window worn to
   the shape of someone who sat there every evening for
   sixty years, and fishing line strung along the
   rafters, looped and looped, patient as a spider's web.

   It smells of woodsmoke long gone out, and lake water,
   and something green.

                                   [space] next",
    "\
   On the table, weighted down with a smooth grey stone,
   you find her journal.

   The leather is cracked soft. Inside: decades of her
   hand, cramped and sure - weather notes, sketches of
   fish with their names and their moods, a running tally
   of mornings that starts the year you were born.

   Tucked in the back cover, a folded note with your
   name on it in the same hand, newer, shakier.

                                   [space] next",
    "\
   \"If you're reading this, I've gone on ahead of you,
   which only means I got to the good water first.

   The lake is quiet now and that's all right - quiet
   isn't the same as dead, it's just resting. Give it
   a reason to wake up. Clear what's grown over. Mend
   what's washed out. Row out past where you can see
   the bottom, when you're ready.

   Everything you need is already here, or close enough
   to it. Go on, now. The fish have been waiting longer
   than you have.
                                        - Wren\"

                                   [space] continue",
];

/// The ending, shown once when the valley is fully restored.
pub static ENDING: &[&str] = &[
    "\
   You wake before the alarm, the way you do now, and
   the valley is already awake ahead of you.

   Herons stalk the clear shallows of the marsh. The
   river runs bright and talkative over its mended bed,
   arguing with every stone. Out on the lake a heron-grey
   mist is lifting off water that finally has somewhere
   to be.

   Stillwater isn't quiet anymore. It just sounds like
   itself.

                                   [space] next",
    "\
   Neighbors you'd never met start finding the path down.

   Someone from the far side of the ridge brings bread
   still warm and stays to fish off the dock till dusk.
   A kid from town asks if the giant in the deep water
   story is true, and you find yourself saying maybe,
   the way Wren would have.

   The cottage still leans a little. You've stopped
   minding. Some things hold their shape by leaning
   into it, not against it.

                                   [space] next",
    "\
   That evening you sit in the chair by the window, and
   for the first time it fits you rather than her.

   You add today's line to the journal, under sixty
   years of her hand and now a season of your own. The
   ink is different. The handwriting will get there.

   Somewhere out past the dark glass, water moves over
   stone, unhurried, in no rush to be anywhere else.
   You think she'd call that a good day's work.

   The lake is awake. You're still not done fishing it.

                                   [space] next",
    "\
   Later, you bank the stove and sit a moment with the
   window dark and the whole valley breathing slow around
   the cottage.

   Somewhere out there a heron shifts its weight on one
   leg. Somewhere a fish turns over just to feel the
   water move. On the sill, the marsh lure catches the
   last of the light, hook long gone, still worth keeping.

   Tomorrow there's more lake to get to know. That's the
   whole of it, really. That's enough.

                                   [space] continue",
];

/// Shown the first time you land a legendary giant from the deep lake (after
/// the boss fight).
pub static LEGEND: &[&str] = &[
    "\
   T H E   D E E P   G I V E S   U P   S O M E T H I N G

   The line goes taut and then the lake itself seems to
   lean back against you, a weight that isn't fighting so
   much as remembering it used to.

   Your arms burn. The rod bends further than wood should
   bend. Somewhere under the boat something vast turns
   over, slow as a thought, and the whole surface flexes
   like skin.

   You think of every evening Wren sat here saying maybe,
   never yes, never no. You think, this is what maybe
   felt like.

                                   [space] next",
    "\
   It comes up out of the dark by degrees, first a
   shadow, then a shape, then more shape than the boat
   feels ready for.

   Scales like old hammered coin. A jaw scarred pale in
   places, healed wrong, healed anyway. One eye, black
   and patient and entirely unbothered, finds yours and
   holds it.

   Neither of you looks away first. It isn't a contest.
   It's just two old things taking stock of each other
   across a lot of water and a little time.

                                   [space] next",
    "\
   You could keep it. Weigh it, measure it, tell the
   story on the dock till you're the age Wren was. Some
   part of you wants to, badly, the way you want to keep
   anything that finally proves itself real.

   Instead you ease the hook free with wet hands gone
   careful, and hold the great cold length of it steady
   in the shallows until it finds its own strength again.

   It doesn't thrash off in a hurry. It just goes, the
   way the lake goes still after a stone, and leaves you
   sitting there soaked and grinning at nothing.

   Some water, Wren used to say, you fish just to keep it
   company. Now you know exactly what she meant.

                                   [space] continue",
];

/// A short story beat shown when a restoration project is funded.
/// project 0 = the marsh, 1 = the river, 2 = the deep lake.
pub fn beat(project: usize) -> &'static [&'static str] {
    match project {
        0 => MARSH_BEAT,
        1 => RIVER_BEAT,
        2 => DEEP_BEAT,
        _ => &[],
    }
}

static MARSH_BEAT: &[&str] = &[
    "\
   C L E A R   T H E   R E E D S

   You spend three days waist-deep in the shallows,
   cutting back reed grown thick as fingers, hauling
   out rot and rusted wire and one entire drowned
   fencepost nobody will ever explain.

   On the third evening the water starts to move again,
   finding its old channels like it remembers the way.
   By morning a heron has already found the new-cut
   shallows and stands there like it owns the place.
   Maybe it does.

                                   [space] continue",
    "\
   Snagged in the last stubborn clump of reed you find
   a lure, sun-faded, hand-tied, hook long since rusted
   to lace.

   You know the pattern before you turn it over - it's
   sketched in the journal, page after page, labeled
   in Wren's cramped hand: \"marsh special, they can't
   say no to it.\"

   You pocket it. Some things aren't for fishing with.
   Some things are just for keeping.

                                   [space] continue",
];

static RIVER_BEAT: &[&str] = &[
    "\
   M E N D   T H E   R I V E R   P A T H

   The old path washed out years before you got here,
   the bank slumped into the current in one long slow
   collapse. You spend a week hauling stone, driving
   posts, building the bank back up handful by handful.

   When the current finally runs clean through the new
   channel, clear over clean gravel instead of choking
   on silt, you can hear it change - a brighter sound,
   like the river clearing its throat.

