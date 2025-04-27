use crate::game_map::GameMap;
use crate::input::Direction;
use std::io::Result;

pub struct Player {
    pub x: usize,
    pub y: usize,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        // TODO: Создать игрока
        unimplemented!()
    }

    pub fn move_in_direction(&mut self, direction: Direction, map: &GameMap) -> Result<()> {
        // TODO: Изменить x/y по направлению
        // TODO: Проверить границы карты
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_new() {
        let player = Player::new(2, 3);
        assert_eq!(player.x, 2);
        assert_eq!(player.y, 3);
    }

    #[test]
    fn test_player_move_up() {
        let mut player = Player::new(2, 2);
        let map = GameMap::new(5, 5);
        player.move_in_direction(Direction::Up, &map).unwrap();
        assert_eq!(player.x, 2);
        assert_eq!(player.y, 1);
    }

    #[test]
    fn test_player_move_down() {
        let mut player = Player::new(2, 2);
        let map = GameMap::new(5, 5);
        player.move_in_direction(Direction::Down, &map).unwrap();
        assert_eq!(player.x, 2);
        assert_eq!(player.y, 3);
    }

    #[test]
    fn test_player_move_left() {
        let mut player = Player::new(2, 2);
        let map = GameMap::new(5, 5);
        player.move_in_direction(Direction::Left, &map).unwrap();
        assert_eq!(player.x, 1);
        assert_eq!(player.y, 2);
    }

    #[test]
    fn test_player_move_right() {
        let mut player = Player::new(2, 2);
        let map = GameMap::new(5, 5);
        player.move_in_direction(Direction::Right, &map).unwrap();
        assert_eq!(player.x, 3);
        assert_eq!(player.y, 2);
    }

    #[test]
    fn test_player_move_out_of_bounds() {
        let mut player = Player::new(4, 4);
        let map = GameMap::new(5, 5);
        player.move_in_direction(Direction::Right, &map).unwrap();
        assert_eq!(player.x, 4);
        assert_eq!(player.y, 4);
        player.move_in_direction(Direction::Down, &map).unwrap();
        assert_eq!(player.x, 4);
        assert_eq!(player.y, 4);
    }
}
