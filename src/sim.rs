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

