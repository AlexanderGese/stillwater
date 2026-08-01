//! The River — flowing water, river fish.
//!
//! A wide, winding river snakes down through grassy banks dotted with
//! trees and rocks. Three plank bridges cross the water at different
//! points so the player can reach either bank and fish from whichever
//! side suits them. A dry path corridor threads from the start, over
//! the bridges, and down to a single open gap in the south wall, which
//! leads out to the Lake.
use crate::areas::{parse_area_water, Area};
use crate::geom::Point;
use crate::season::WaterType;

pub fn river_area() -> Area {
    let template = "\
####################################################################\n\
#...................~~.......................o.....................#\n\
#....,,,,,,,,,.......~~.............................T.o.........T..#\n\
#......o.o...,.........~~.......................................T..#\n\
#............,..........T~~........................................#\n\
#..........T.,.............~~~T......o....T.o......................#\n\
#............,,,,,,...........xx..............oT...................#\n\
#.............................T.~~...............................T.#\n\
#......T........o...o...........~~.................................#\n\
#...T...................T.......~~.................................#\n\
#.................T...........~~~.........................o........#\n\
#............................~~..........o.............T........o..#\n\
#............T...........T.~~..............................T.......#\n\
#.......................xx..,,,,,,,................oT..............#\n\
#.................oT.~~...........,.....................T..........#\n\
#..................~~~.....T......,................................#\n\
#.o...............~~.......o......,.........................T......#\n\
#...T...........T~~..T............,................................#\n\
#.........T.T....~~..........T....,.................T..............#\n\
#T......TT.T....T.~~..............,.............................o..#\n\
#..................xxx............,..o......TT.....................#\n\
#.....................~~..........,................................#\n\
#.......................~~........,...........o....................#\n\
#.......................o.~~......,.T........................o.....#\n\
#.......o...................~~....,.........T.T....................#\n\
##################################,#################################";
    parse_area_water("The River", Point::new(4, 2), template, Some(WaterType::River))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point;

    #[test]
    fn river_area_is_valid() {
        let a = river_area();
        assert!(a.map.w >= 60 && a.map.h >= 24);
        assert!(a.map.walkable(a.start));

        let mut has_water = false;
        for y in 0..a.map.h {
            for x in 0..a.map.w {
                if a.map.get(Point::new(x, y)).is_water() {
                    has_water = true;
                }
            }
        }
        assert!(has_water);

        // The south edge (bottom border) has an open, walkable crossing
        // point down to the Lake.
        let mut south_open = false;
        for x in 0..a.map.w {
            if a.map.walkable(Point::new(x, a.map.h - 1)) {
                south_open = true;
            }
        }
        assert!(south_open);
    }
}
