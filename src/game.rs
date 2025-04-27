use crate::ai::AI;
use crate::apple::Apple;
use crate::game_map::GameMap;
use crate::input::GameAction;
use crate::player::Player;
use std::io::Result;

pub struct Game {
    map: GameMap,
    players: Vec<Player>,
    ais: Vec<AI>,
    apple: Apple,
}

impl Game {
    pub fn new(
        map_width: usize,
        map_height: usize,
        player_x: usize,
        player_y: usize,
    ) -> Result<Self> {
        if player_x >= map_width || player_y >= map_height {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Player out of bounds",
            ));
        }
        let map = GameMap::new(map_width, map_height);
        let player = Player::new(player_x, player_y);
        let ais = vec![AI::new(1, 1)];
        let apple = Apple::new(&map);
        Ok(Game {
            map,
            players: vec![player],
            ais,
            apple,
        })
    }

    pub fn update(&mut self, action: GameAction) -> Result<()> {
        if let GameAction::Direction(dir) = action {
            self.players[0].move_in_direction(dir, &self.map)?;
        }
        for ai in &mut self.ais {
            ai.random_move(&self.map)?;
        }
        // Проверка: съел ли игрок яблочко
        let player = &self.players[0];
        if player.x == self.apple.x && player.y == self.apple.y {
            // В будущем: увеличить здоровье и сгенерировать новое яблочко
            println!("Apple collected! (Health increase coming soon)");
        }
        Ok(())
    }
    pub fn get_map(&self) -> &GameMap {
        &self.map
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn get_ais(&self) -> &Vec<AI> {
        &self.ais
    }

    pub fn get_apple(&self) -> &Apple {
        &self.apple
    }
}
