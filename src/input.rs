use crate::game::Action;
use crate::geom::Dir;
use crossterm::event::KeyCode;

pub fn key_to_action(code: KeyCode) -> Option<Action> {
    match code {
        KeyCode::Char('w') | KeyCode::Up => Some(Action::Move(Dir::North)),
        KeyCode::Char('s') | KeyCode::Down => Some(Action::Move(Dir::South)),
        KeyCode::Char('a') | KeyCode::Left => Some(Action::Move(Dir::West)),
        KeyCode::Char('d') | KeyCode::Right => Some(Action::Move(Dir::East)),
        KeyCode::Char('e') | KeyCode::Enter | KeyCode::Char(' ') => Some(Action::Interact),
        KeyCode::Char('j') => Some(Action::OpenJournal),
        KeyCode::Char('?') => Some(Action::OpenHelp),
        KeyCode::Char('z') => Some(Action::Wait), // pass a little time
        KeyCode::Char(c @ '1'..='9') => Some(Action::Buy(c as u8 - b'0')),
        KeyCode::Esc => Some(Action::Back),
        KeyCode::Char('q') => Some(Action::Quit),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Action;
    use crate::geom::Dir;
    use crossterm::event::KeyCode;
    #[test]
    fn maps_keys() {
        assert_eq!(key_to_action(KeyCode::Char('w')), Some(Action::Move(Dir::North)));
        assert_eq!(key_to_action(KeyCode::Up), Some(Action::Move(Dir::North)));
        assert_eq!(key_to_action(KeyCode::Char('d')), Some(Action::Move(Dir::East)));
        assert_eq!(key_to_action(KeyCode::Char('e')), Some(Action::Interact));
        assert_eq!(key_to_action(KeyCode::Char('j')), Some(Action::OpenJournal));
        assert_eq!(key_to_action(KeyCode::Char('1')), Some(Action::Buy(1)));
        assert_eq!(key_to_action(KeyCode::Char('6')), Some(Action::Buy(6)));
        assert_eq!(key_to_action(KeyCode::Char('z')), Some(Action::Wait));
        assert_eq!(key_to_action(KeyCode::Esc), Some(Action::Back));
        assert_eq!(key_to_action(KeyCode::Char('q')), Some(Action::Quit));
        assert_eq!(key_to_action(KeyCode::Char('x')), None);
    }
}
