use crate::game_map::GameMap;
use crate::input::Direction;
use rand::{Rng, rng};
use std::io::Result;

pub struct AI {
    pub x: usize,
    pub y: usize,
    pub health: i32,
}

impl AI {
    pub fn new(x: usize, y: usize) -> Self {
        AI { x, y, health: 2 }
    }

    pub fn random_move(&mut self, map: &GameMap) -> Result<()> {
        let mut rng = rng();
        let dir = match rng.random_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        };
        let (new_x, new_y) = match dir {
            Direction::Up => (self.x, self.y.saturating_sub(1)),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x.saturating_sub(1), self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        if new_x < map.width && new_y < map.height {
            self.x = new_x;
            self.y = new_y;
        }
        Ok(())
    }
}
