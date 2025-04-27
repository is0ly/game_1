use crossterm::event::{Event, KeyCode, poll, read};
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum GameAction {
    Direction(Direction),
    Exit,
}

pub fn read_input() -> std::io::Result<Option<GameAction>> {
    if poll(Duration::from_millis(16))? {
        match read()? {
            Event::Key(event) => Ok(match event.code {
                KeyCode::Up => Some(GameAction::Direction(Direction::Up)),
                KeyCode::Down => Some(GameAction::Direction(Direction::Down)),
                KeyCode::Left => Some(GameAction::Direction(Direction::Left)),
                KeyCode::Right => Some(GameAction::Direction(Direction::Right)),
                KeyCode::Esc | KeyCode::Char('q') => Some(GameAction::Exit),
                _ => None,
            }),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key_to_action(key: KeyCode) -> Option<GameAction> {
        match key {
            KeyCode::Up => Some(GameAction::Direction(Direction::Up)),
            KeyCode::Down => Some(GameAction::Direction(Direction::Down)),
            KeyCode::Left => Some(GameAction::Direction(Direction::Left)),
            KeyCode::Right => Some(GameAction::Direction(Direction::Right)),
            KeyCode::Esc | KeyCode::Char('q') => Some(GameAction::Exit),
            _ => None,
        }
    }

    #[test]
    fn test_key_to_action_up() {
        let action = key_to_action(KeyCode::Up);
        assert_eq!(action, Some(GameAction::Direction(Direction::Up)));
    }

    #[test]
    fn test_key_to_action_down() {
        let action = key_to_action(KeyCode::Down);
        assert_eq!(action, Some(GameAction::Direction(Direction::Down)));
    }

    #[test]
    fn test_key_to_action_left() {
        let action = key_to_action(KeyCode::Left);
        assert_eq!(action, Some(GameAction::Direction(Direction::Left)));
    }

    #[test]
    fn test_key_to_action_right() {
        let action = key_to_action(KeyCode::Right);
        assert_eq!(action, Some(GameAction::Direction(Direction::Right)));
    }

    #[test]
    fn test_key_to_action_exit_esc() {
        let action = key_to_action(KeyCode::Esc);
        assert_eq!(action, Some(GameAction::Exit));
    }

    #[test]
    fn test_key_to_action_exit_q() {
        let action = key_to_action(KeyCode::Char('q'));
        assert_eq!(action, Some(GameAction::Exit));
    }

    #[test]
    fn test_key_to_action_unknown() {
        let action = key_to_action(KeyCode::Char('x'));
        assert_eq!(action, None);
    }
}
