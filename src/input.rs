use crossterm::event::{KeyCode, poll, read, Event};
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
    // TODO: Проверить наличие ввода с помощью poll (таймаут 16 мс)
    // TODO: Считать событие с помощью read
    // TODO: Для Event::Key преобразовать KeyCode:
    // - Up/Down/Left/Right -> GameAction::Direction
    // - Esc/Char('q') -> GameAction::Exit
    // - Игнорировать другие события
    // TODO: Вернуть None, если ввода нет
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key_to_action(key: KeyCode) -> Option<GameAction> {
        // TODO: Преобразовать KeyCode в GameAction
        unimplemented!()
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
