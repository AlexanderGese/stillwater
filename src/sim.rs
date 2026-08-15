use crate::game::{Action, Game};
use crate::geom::Dir;

pub fn run_script(actions: &[Action]) -> Game {
    let mut g = Game::new();
    for &a in actions {
        g.apply(a);
    }
    g
}

pub fn walk(path: &str) -> Game {
    let actions: Vec<Action> = path
        .chars()
        .filter_map(|c| match c {
            'w' => Some(Action::Move(Dir::North)),
            's' => Some(Action::Move(Dir::South)),
            'a' => Some(Action::Move(Dir::West)),
            'd' => Some(Action::Move(Dir::East)),
            _ => None,
        })
        .collect();
    run_script(&actions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::Point;
    #[test]
    fn scripted_walk_moves_player() {
        // From start, move East twice then South once (all onto walkable yard tiles).
        let g0 = crate::game::Game::new();
        let start = g0.player.pos;
        let g = walk("dds");
        assert_eq!(g.player.pos, Point::new(start.x + 2, start.y + 1));
    }
}
