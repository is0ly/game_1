use crate::player::Player;
use crate::game_map::GameMap;
use crate::input::{GameAction, Direction};
use std::io::Result;

pub struct Game {
    map: GameMap,
    players: Vec<Player>,
}

impl Game {
    pub fn new(map_width: usize, map_height: usize, player_x: usize, player_y: usize) -> Result<Self> {
        // TODO: Создать GameMap
        // TODO: Создать Player
        // TODO: Проверить валидность позиции игрока
        // TODO: Вернуть Game с одним игроком
        unimplemented!()
    }

    pub fn update(&mut self, action: GameAction) -> Result<()> {
        // TODO: Если GameAction::Direction, переместить первого игрока
        // TODO: Игнорировать Exit
        unimplemented!()
    }

    pub fn get_map(&self) -> &GameMap {
        // TODO: Вернуть карту
        unimplemented!()
    }

    pub fn get_players(&self) -> &Vec<Player> {
        // TODO: Вернуть игроков
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_new_valid() {
        let result = Game::new(5, 5, 2, 2);
        assert!(result.is_ok(), "Game::new должен создавать игру");
        let game = result.unwrap();
        assert_eq!(game.get_map().width, 5);
        assert_eq!(game.get_map().height, 5);
        assert_eq!(game.get_players().len(), 1);
        assert_eq!(game.get_players()[0].x, 2);
        assert_eq!(game.get_players()[0].y, 2);
    }

    #[test]
    fn test_game_new_invalid_position() {
        let result = Game::new(5, 5, 6, 2);
        assert!(result.is_err(), "Game::new должен отклонять x=6");
    }

    #[test]
    fn test_game_new_invalid_size() {
        let result = Game::new(0, 5, 2, 2);
        assert!(result.is_err(), "Game::new должен отклонять width=0");
    }

    #[test]
    fn test_game_update_move_right() {
        let mut game = Game::new(5, 5, 2, 2).unwrap();
        let action = GameAction::Direction(Direction::Right);
        game.update(action).unwrap();
        assert_eq!(game.get_players()[0].x, 3);
        assert_eq!(game.get_players()[0].y, 2);
    }

    #[test]
    fn test_game_update_move_out_of_bounds() {
        let mut game = Game::new(5, 5, 4, 2).unwrap();
        let action = GameAction::Direction(Direction::Right);
        game.update(action).unwrap();
        assert_eq!(game.get_players()[0].x, 4);
        assert_eq!(game.get_players()[0].y, 2);
    }

    #[test]
    fn test_game_update_exit_ignored() {
        let mut game = Game::new(5, 5, 2, 2).unwrap();
        let action = GameAction::Exit;
        game.update(action).unwrap();
        assert_eq!(game.get_players()[0].x, 2);
        assert_eq!(game.get_players()[0].y, 2);
    }
}
