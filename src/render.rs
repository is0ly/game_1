use crate::game::Game;
use crate::game_map::GameMap;
use crate::player::Player;
use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::Result;

pub fn render_game(game: &Game) -> Result<()> {
    // TODO: Получить карту и игроков
    // TODO: Вызвать render_map
    unimplemented!()
}

pub fn render_map(map: &GameMap, players: &[&Player]) -> Result<()> {
    // TODO: Очистить экран
    // TODO: Для каждой клетки вывести '@' (игрок) или tile
    // TODO: Использовать queue! и execute!
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_game_no_error() {
        let game = Game::new(5, 5, 2, 2).unwrap();
        let result = render_game(&game);
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_map_no_error() {
        let map = GameMap::new(5, 5);
        let player = Player::new(2, 2);
        let result = render_map(&map, &[&player]);
        assert!(result.is_ok());
    }
}
