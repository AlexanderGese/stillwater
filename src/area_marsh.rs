//! The Marsh — murky water, reeds, marsh fish. A sprawling wetland: broad
//! pools of murky water broken by scattered tufts of grassy ground, loose
//! drifts of reeds, and a handful of plank walkways so the player can pick a
//! path out to the fishing spots. East edge opens onto the Homestead.
use crate::areas::{parse_area_water, Area};
use crate::geom::Point;
use crate::season::WaterType;

pub fn marsh_area() -> Area {
    let template = "\
####################################################################\n\
#~~~~T~~~~xx~~~~~~~~~~~~~~~~~~~~~~x~~~~~~~~~~~~~~~~~~~~~~~xx~~~~~~~#\n\
#~~T~~~~~~xx~~T~~~~~~~~~~~~~~~T~~~x~~~~~~~~~~~~~~~~~~~~~~~xx~~~~~~~#\n\
#~~~~.~T~~xxT~~~T,.~~~~xx~T~~~~T~~x~~~~~T~~T~~~xx~~~~~~~~~xx,.~~~~~#\n\
#~~~.,.~~~xx~~~~~.~~~~~xxT~~~~T.~~x~~~T~~~~~..~xx~~~~o~~~~xx.T~~~~~#\n\
#~~~~.~~~~xx~~~~~~~~~~~xxT~~~T~,.~x~~~~~~T~~.~~xx~~~~~~~.~xx~~~TT~~#\n\
#~~~~~~~~~xx~~~~~~~~~~Txx~~T~~~~.~x~~~~~~o~~~~~xx~~~~~~.,.xx~~T~~~~#\n\
#~~~~~~~~~xx~~,.~~~~~~~xx~~~~~~~~~x~~~~~~~~~~~~xx~~~T~~~.~xx~~~~~~~#\n\
#~~~~~~~~~xx~~.~~~~~~~Txx~o~~~~~~~x~~~~~~~~~~~~xxT~~~~~~~~xx~~~~~~~#\n\
#~~~~~~~T~xx~~~~~~~~T~~~.~~~~~~~~~xT~~~~~~~~~~~xx.~T~~T~~~xx~~~~~~~#\n\
#~~~~~~o~~xx~~~~~~~~~~~...~~~~~~~~x~~~T~~~~~~T~xx,.~T~~~~~xx~T~~~~~#\n\
#~~~~~~~~~xx~~~~~~~~~~~~.~~~~~~~~~x~~T~~~~~~T~Txx~.~~~~~~~xxT~~~~~~#\n\
#~~,,,~,,~xx~~~,~~~~,,~xx~~~,,~~~~x~,T~~~,,~~T~xx,~~~~,,,,xx~~~~,,~#\n\
#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,\n\
#~,~~,~~~~~,,~~~~~,~,,~xx~~,,~~~,~~~~~~~~,~,~~~xx,~~~,,~~~~~~~~o~,,#\n\
#~~~~~~~~~~~~~~~~xT~~~~xx~~~~~~~~~~~.~~~x~~~~~~xx~~~~~~~~~~~~~~~~~~#\n\
#~~~~~T~~.~~~~~~~x~~T~~xx~~~~o~~~~~.,.~~x~~~~~~~~~~~~~T~~~~~~~~~~~~#\n\
#~~~~~~~~..~~~~~~x~~~~~xx~~~~~~~~~~~.~~~x~~~~~~~~T~~~~~T~~~~~~,.~~~#\n\
#~~~~~~~T~.~~~~~ox~~~~~xx~~~~~~~~~~~~~~~x~~~~~~~~~~~~~~~~~~~~~.~~~~#\n\
#~~~~~~T~~~~~~~~~x~~~.~xx~~T~~~~~~~~~~~~x~~~~~oxx~~~~~~~~~~~~~~~T~~#\n\
#~~~~.T~~~~~~~~~~x~~.,.xx~~~~~~~~~~~~~~~x~~~~~~xx~~~~~~~~~~~~~~~~~~#\n\
#~~~~..~~~~~~~~~~x~~~.~xx~~~~T~~~,.~~o~~x~~~~~~xx~~~~~~~~~~~~~~T~~~#\n\
#~~~~~.~~~~~~~~T~x~~~~~xx~~~~~~~~.~~~~~~xT~~~~~xx~~~~~~~~~~~~~~~~T~#\n\
#~~~~~~~~~~~~~~~~x~~~~~xx~~~~~T~~~~~~~~~x~~~~~~xx~~~~~~~~~~~~~~~~~~#\n\
#~~~~~~~~~~~~~T~~x~~~~~xx~~~~~~~~~~~~~~~x~~~~~~xx~~~~~~~~~~~~~~~~~~#\n\
####################################################################";
    parse_area_water("The Marsh", Point::new(6, 13), template, Some(WaterType::Marsh))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point;

    #[test]
    fn marsh_area_is_valid() {
        let a = marsh_area();
        assert!(a.map.w >= 60 && a.map.h >= 22);
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

