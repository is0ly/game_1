mod ai;
mod apple;
mod game;
mod game_map;
mod input;
mod player;
mod render;

use crate::game::Game;
use crate::input::GameAction;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Настройка терминала
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, Hide)?;

    let mut game = Game::new(10, 10, 5, 5)?;

    loop {
        // Обработка ввода
        let input = crate::input::read_input()?;

        if let Some(GameAction::Exit) = input {
            break;
        }
        if let Some(action) = input {
            game.update(action)?;
        }

        // Рендеринг
        crate::render::render_game(&game)?;

        // Контроль FPS
        thread::sleep(Duration::from_millis(16));
    }

    // Очистка терминала
    execute!(std::io::stdout(), LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}
