pub struct GameMap {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Vec<char>>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        // TODO: Создать карту с '.' в клетках
        // TODO: Проверить width, height > 0
        unimplemented!()
    }

    pub fn get_tiles(&self) -> &Vec<Vec<char>> {
        // TODO: Вернуть tiles
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_map_new() {
        let map = GameMap::new(5, 5);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 5);
        assert_eq!(map.get_tiles().len(), 5);
        assert_eq!(map.get_tiles()[0].len(), 5);
        assert_eq!(map.get_tiles()[0][0], '.');
    }

    #[test]
    fn test_game_map_new_invalid_width() {
        let result = std::panic::catch_unwind(|| GameMap::new(0, 5));
        assert!(result.is_err(), "Width=0 должен вызывать панику");
    }

    #[test]
    fn test_game_map_new_invalid_height() {
        let result = std::panic::catch_unwind(|| GameMap::new(5, 0));
        assert!(result.is_err(), "Height=0 должен вызывать панику");
    }
}
