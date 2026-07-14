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

