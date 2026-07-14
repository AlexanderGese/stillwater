# Stillwater Farm

A cozy terminal **fishing** game in Rust. You move onto a quiet run-down lakeside
and settle in — cast across the seasons, land fish, fill your journal, and slowly
upgrade your tackle. Chill, low-stakes, and still worth a five-minute sitting.

Built with `crossterm` + the standard library only. No other dependencies.

## Play

```
cargo run
```

Your progress autosaves to `.stillwater.save` in the working directory and loads
automatically next time.

The game opens on a **main menu** (New Game / Continue / How to Play / Quit).

**New here?** A new game opens with the story of how you came to Stillwater, then
a short how-to, and a hint line at the bottom of the screen always tells you what
to do next. Press `?` any time for the full controls.

**A story, too.** You've inherited the run-down lakeside from your late great-aunt
Wren. Restoring the marsh, the river, and the deep lake plays out a quiet story
told through her old fishing journal, ending when the whole valley is alive again.

## Controls

| Keys | Does |
|------|------|
| `w a s d` / arrows | walk |
| `e` / space / enter | cast at the water · set the hook · reel · sleep at the bed (`B`) · open the shop (`H`) |
| `w` / `s` (during a fight) | **reel** / **ease** — ease off while the fish is *darting*, reel when it's calm |
| `j` | open the fish journal |
| `z` | wait a little while |
| `1`–`9` | buy in the shop / fund a project on the restoration board |
| `esc` | close a menu |
| `q` | quit |

Walk into an open edge of a map to travel to the next area.

## The loop

Walk out onto the dock, face the lake, and cast. Wait for a bite, set the hook,
then reel the fish in without letting the line snap — watch the tension bar and
ease off when it darts. Landing a fish pays out gold and records it in your
**journal** (27 species across the four seasons, dawn-to-night windows, and four
water types).

Spend your gold at the tackle shop (`H`): better **rods** cast farther into
deeper water and fight fish more forgivingly, and better **bait** draws more
bites. Sleep at your bed (`B`) to roll into the next day — the calendar advances
through 28-day seasons, and each morning brings fresh weather that changes what's
biting (rain and storms stir the fish up; frost and snow slow them down).

## The valley

Six hand-drawn areas linked at their edges, rendered with smart box-drawing
walls and a camera that follows you across the big maps:

- **Homestead** — your cabin, shop, ship bin, and a little pond to fish.
- **Stillwater Lake** — the wide open water (shallow shore, deep middle).
- **Town** — the **restoration notice board** (`N`).
- **The Marsh**, **The River**, **The Deep Lake** — reached by funding
  restoration projects at the board (each opens a new kind of water with its
  own fish).

Fund **Clear the Reeds → Mend the River Path → Repair the Rowboat** with your
earnings to open the marsh, the river, and finally the deep lake where the
biggest fish lurk.

## What's inside

- Six big linked, hand-authored tile areas with edge transitions, box-drawing
  walls, a scrolling camera, and optional colour.
- A main menu, a settings screen (hints / colour / guided tutorial), a paged
  story + how-to, a step-by-step guided tutorial for your first day, a
  context-aware hint line, and a `?` help screen.
- Legendary deep-lake giants fought as multi-phase **boss fights**, with their
  own story moment.
- A walkable world with an in-game clock (6am → 2am), energy, and sleep.
- Four seasons × seven weather types, all seeded and deterministic.
- 45 hand-authored fish species (incl. deep-lake legendaries); 6 baits; 4 rod
  tiers; 3 restoration projects.
- A tension-bar reel minigame, a fish journal, a shop, and hand-rolled versioned
  save/load — no `serde`.
