use crate::game_map::GameMap;
use rand::{Rng, rng};

pub struct Apple {
    pub x: usize,
    pub y: usize,
}

impl Apple {
    pub fn new(map: &GameMap) -> Self {
        let mut rng = rng();
        let x = rng.random_range(0..map.width);
        let y = rng.random_range(0..map.height);
        Apple { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_map::GameMap;

    #[test]
    fn test_apple_spawn() {
        let map = GameMap::new(5, 5);
        let apple = Apple::new(&map);
        assert!(apple.x < map.width);
        assert!(apple.y < map.height);
    }
}
