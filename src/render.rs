use crate::ai::AI;
use crate::apple::Apple;
use crate::game::Game;
use crate::game_map::GameMap;
use crate::player::Player;
use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, size},
};
use std::io::Result;

pub fn render_game(game: &Game) -> Result<()> {
    let map = game.get_map();
    let players = game.get_players();
    let player_refs: Vec<&Player> = players.iter().collect();
    let ais = game.get_ais();
    let ai_refs: Vec<&AI> = ais.iter().collect();
    let apple = game.get_apple();
    let _ = render_map(map, &player_refs, &ai_refs, apple);

    let player = &players[0];
    let hud_text = format!(
        "Player: ({}, {}) | Health: {}",
        player.x, player.y, player.health
    );
    queue!(std::io::stdout(), MoveTo(0, 0), Print(hud_text))?;
    execute!(std::io::stdout())?;
    Ok(())
}

pub fn render_map(map: &GameMap, players: &[&Player], ais: &[&AI], apple: &Apple) -> Result<()> {
    let (terminal_width, terminal_height) = size()?;
    let map_display_width = (map.width * 4) as u16;
    let map_display_height = (map.height * 2) as u16;
    let offset_x = (terminal_width.saturating_sub(map_display_width)) / 2;
    let offset_y = (terminal_height.saturating_sub(map_display_height)) / 2;

    execute!(std::io::stdout(), Clear(ClearType::All))?;
    let tiles = map.get_tiles();
    for (y, row) in tiles.iter().enumerate().take(map.height) {
        for (x, &tile) in row.iter().enumerate().take(map.width) {
            let display_x = (x * 4) as u16 + offset_x;
            let display_y = (y * 2) as u16 + offset_y;
            let symbol = if players.iter().any(|p| p.x == x && p.y == y) {
                "@"
            } else if ais.iter().any(|ai| ai.x == x && ai.y == y) {
                "E"
            } else if apple.x == x && apple.y == y {
                "$"
            } else if tile == '.' {
                "."
            } else if tile == '#' {
                "##"
            } else {
                panic!("Unknown tile: {}", tile);
            };
            queue!(
                std::io::stdout(),
                MoveTo(display_x, display_y),
                Print(symbol)
            )?;
        }
        queue!(
            std::io::stdout(),
            MoveTo(offset_x, (y * 2 + 1) as u16 + offset_y),
            Print(" ")
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_game_no_error() {
        // Создаём игру с картой 5x5 и игроком на (2,2)
        let game = Game::new(5, 5, 2, 2).unwrap();
        let result = render_game(&game);
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_map_no_error() {
        // Создаём карту 5x5
        let map = GameMap::new(5, 5);
        // Создаём игрока на (2,2)
        let player = Player::new(2, 2);
        // Создаём ИИ на (1,1)
        let ai = AI::new(1, 1);
        // Создаём яблочко (используем Apple::new для случайной позиции)
        let apple = Apple::new(&map);
        // Передаём все необходимые параметры в render_map
        let result = render_map(&map, &[&player], &[&ai], &apple);
        assert!(result.is_ok());
    }
}
