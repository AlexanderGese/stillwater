//! Hand-rolled, versioned save/load (no serde). A save is a small line-based
//! text file: `key value` per line, with a leading `stillwater <version>` tag.

use crate::fish::{self, Catch};
use crate::game::Game;
use crate::journal::Journal;
use crate::season::Season;
use crate::weather::Weather;
use std::fs;
use std::io::Write;

const VERSION: u32 = 1;

