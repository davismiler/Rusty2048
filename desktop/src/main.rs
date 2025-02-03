// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusty2048_core::{Game, GameConfig, Direction};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
struct GameState {
    board: Vec<Vec<u32>>,
    score: u32,
    best_score: u32,
    moves: u32,
    game_state: String,
}

struct GameManager {
    game: Game,
}

impl GameManager {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = GameConfig::default();
        let game = Game::new(config)?;
        Ok(GameManager { game })
    }
    
    fn get_state(&self) -> GameState {
        let board = self.game.board();
        let size = board.size();
        let mut board_data = vec![vec![0u32; size]; size];
        
        for row in 0..size {
            for col in 0..size {
                if let Ok(tile) = board.get_tile(row, col) {
                    board_data[row][col] = tile.value;
                }
            }
        }
        
        let game_state = match self.game.state() {
            rusty2048_core::GameState::Playing => "playing",
            rusty2048_core::GameState::Won => "won",
            rusty2048_core::GameState::GameOver => "game_over",
        };
        
        GameState {
            board: board_data,
            score: self.game.score().current(),
            best_score: self.game.score().best(),
            moves: self.game.moves(),
            game_state: game_state.to_string(),
        }
    }
}

#[tauri::command]
fn make_move(state: State<'_, GameManager>, direction: String) -> Result<GameState, String> {
    let dir = match direction.as_str() {
        "up" => Direction::Up,
        "down" => Direction::Down,
        "left" => Direction::Left,
        "right" => Direction::Right,
        _ => return Err("Invalid direction".to_string()),
    };
    
    let mut game_manager = state.inner().clone();
    game_manager.game.make_move(dir).map_err(|e| e.to_string())?;
    Ok(game_manager.get_state())
}

#[tauri::command]
fn get_state(state: State<'_, GameManager>) -> GameState {
    state.inner().get_state()
}

#[tauri::command]
fn new_game(state: State<'_, GameManager>) -> Result<GameState, String> {
    let mut game_manager = state.inner().clone();
    game_manager.game.new_game().map_err(|e| e.to_string())?;
    Ok(game_manager.get_state())
}

fn main() {
    let game_manager = GameManager::new().expect("Failed to create game");
    
    tauri::Builder::default()
        .manage(game_manager)
        .invoke_handler(tauri::generate_handler![
            make_move,
            get_state,
            new_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
