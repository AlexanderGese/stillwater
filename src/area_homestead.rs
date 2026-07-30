//! The Homestead — your cabin, shop, ship bin, a big pond to fish, and the
//! paths out to the rest of the valley. A big, cozy hub map: a walled cabin
//! (bed inside), a shop, ship bin and notice board on the plaza, a dock and a
//! long bridge crossing the pond, winding paths, and trees/rocks scattered
//! for detail. Three border gaps lead out: north to Town, east to the Lake,
//! west to the Marsh.

use crate::areas::{parse_area_water, Area};
use crate::geom::Point;

pub fn homestead_area() -> Area {
    let template = "\
##################################,,,,##################################\n\
#..................................,,..................................#\n\
#..#############...................,,..................................#\n\
#..#...........#..T...T............,,.......T.......T...........T......#\n\
#..#.BB........#............T......,,...........T...........T..........#\n\
#..#...........#....T...o..........,,...o...o..........T...............#\n\
#..#...........#T................T.,,..................................#\n\
#..#...........#..T................,,.............o....................#\n\
#..#...........#............o......,,.......T..................T.......#\n\
#..######,######...................,,,,,,,,,,,,,,,,,,,,,,,,............#\n\
#........,.........................,,.........~~~~~~~~~~~~x~~~~~~~~~~..#\n\
#........,.........................,,.........~~~~~~~~~~~~x~~~~~~~~~~..#\n\
#..T.....,........T.o.....o.....o...o.......T.~~~~~~~~~~~~x~~~~~~~~~~..#\n\
#........,T...................................~~~~\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}x\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}~~~~..,\n\
#.....T..,....................................~~~~\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248},xxxxxxxxxx,,,\n\
#........,..........,....................,=======~\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}x\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}~~~~..#\n\
#........,..........,....................,....~~~~\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}x\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}~~~~..#\n\
#....o...,o....o....,...HH....S.......N..,....~~~~\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}x\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}~~~~..#\n\
#........,..........,,,,,,,,,,,,,,,,,,,,,,,,,,~~~~\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}x\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}~~~~..#\n\
#........,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,~~~~\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}x\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}\u{2248}~~~~..#\n\
#...........T.......,.........................~~~~~~~~~~~~x~~~~~~~~~~..#\n\
,...................,.........................~~~~~~~~~~~~x~~~~~~~~~~..#\n\
,,,,,,,,,,,,,,,,,,,,,.........................~~~~~~~~~~~~x~~~~~~~~~~..#\n\
#.................T....................................................#\n\
#..T..........T..........T....o............o..o........T........T......#\n\
#.......T...o.......................o.............o.........T..........#\n\
#......................................................................#\n\
########################################################################";
    parse_area_water("Homestead", Point::new(9, 11), template, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point;

    #[test]
    fn builds_and_is_big_enough() {
        let a = homestead_area();
        assert!(a.map.w >= 70 && a.map.h >= 26);
    }

    #[test]
    fn start_is_walkable() {
        let a = homestead_area();
        assert!(a.map.walkable(a.start));
    }

    #[test]
    fn has_shallow_water() {
        let a = homestead_area();
        let mut has_shallow = false;
        for y in 0..a.map.h {
            for x in 0..a.map.w {
                if a.map.get(Point::new(x, y)) == crate::tile::Tile::ShallowWater {
                    has_shallow = true;
                }
            }
        }
        assert!(has_shallow);
    }

    #[test]
    fn east_edge_has_a_walkable_gap_to_the_lake() {
        let a = homestead_area();
        let mut found = false;
        for y in 0..a.map.h {
            if a.map.walkable(Point::new(a.map.w - 1, y)) {
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn north_edge_has_a_walkable_gap_to_town() {
        let a = homestead_area();
        let mut found = false;
        for x in 0..a.map.w {
            if a.map.walkable(Point::new(x, 0)) {
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn west_edge_has_a_walkable_gap_to_the_marsh() {
        let a = homestead_area();
        let mut found = false;
        for y in 0..a.map.h {
            if a.map.walkable(Point::new(0, y)) {
                found = true;
            }
        }
        assert!(found);
    }
}
