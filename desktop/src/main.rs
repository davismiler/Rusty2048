// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusty2048_core::{Game, GameConfig, Direction};
use rusty2048_shared::Theme;
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct SetThemeArgs {
    #[serde(alias = "themeName")]
    theme_name: String,
}

#[derive(Serialize, Deserialize)]
struct GameState {
    board: Vec<Vec<u32>>,
    score: u32,
    best_score: u32,
    moves: u32,
    game_state: String,
    max_tile: u32,
    can_undo: bool,
    theme: Theme,
}

struct GameManager {
    game: Game,
    theme: Theme,
}

impl GameManager {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = GameConfig::default();
        let game = Game::new(config)?;
        let theme = Theme::default();
        Ok(GameManager { game, theme })
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
            max_tile: self.game.board().max_tile(),
            can_undo: true, // TODO: Add public method to check undo availability
            theme: self.theme.clone(),
        }
    }
}

#[tauri::command]
async fn make_move(state: State<'_, Arc<Mutex<GameManager>>>, direction: String) -> Result<GameState, String> {
    let dir = match direction.as_str() {
        "up" => Direction::Up,
        "down" => Direction::Down,
        "left" => Direction::Left,
        "right" => Direction::Right,
        _ => return Err("Invalid direction".to_string()),
    };

    let mut game_manager = state.lock().map_err(|_| "lock poisoned".to_string())?;
    game_manager.game.make_move(dir).map_err(|e| e.to_string())?;
    Ok(game_manager.get_state())
}

#[tauri::command]
async fn get_state(state: State<'_, Arc<Mutex<GameManager>>>) -> Result<GameState, String> {
    let game_manager = state.lock().map_err(|_| "lock poisoned".to_string())?;
    Ok(game_manager.get_state())
}

#[tauri::command]
async fn new_game(state: State<'_, Arc<Mutex<GameManager>>>) -> Result<GameState, String> {
    let mut game_manager = state.lock().map_err(|_| "lock poisoned".to_string())?;
    game_manager.game.new_game().map_err(|e| e.to_string())?;
    Ok(game_manager.get_state())
}

#[tauri::command]
async fn undo(state: State<'_, Arc<Mutex<GameManager>>>) -> Result<GameState, String> {
    let mut game_manager = state.lock().map_err(|_| "lock poisoned".to_string())?;
    game_manager.game.undo().map_err(|e| e.to_string())?;
    Ok(game_manager.get_state())
}

#[tauri::command]
async fn set_theme(state: State<'_, Arc<Mutex<GameManager>>>, args: SetThemeArgs) -> Result<GameState, String> {
    let mut mgr = state.lock().map_err(|_| "lock poisoned".to_string())?;
    if let Some(theme) = Theme::by_name(&args.theme_name) {
        mgr.theme = theme;
        Ok(mgr.get_state())
    } else {
        Err("Invalid theme name".into())
    }
}

#[tauri::command]
async fn get_available_themes() -> Vec<String> {
    Theme::all_themes().iter().map(|t| t.name.clone()).collect()
}

#[tauri::command]
async fn get_stats(state: State<'_, Arc<Mutex<GameManager>>>) -> Result<serde_json::Value, String> {
    let game_manager = state.lock().map_err(|_| "lock poisoned".to_string())?;
    let stats = game_manager.game.stats();
    Ok(serde_json::json!({
        "duration": stats.duration,
        "max_tile": game_manager.game.board().max_tile(),
        "moves": game_manager.game.moves(),
        "score": game_manager.game.score().current(),
        "best_score": game_manager.game.score().best()
    }))
}

#[tauri::command]
async fn test_connection() -> Result<String, String> {
    Ok("Tauri connection successful!".to_string())
}

fn main() {
    let game_manager = Arc::new(Mutex::new(GameManager::new().expect("Failed to create game")));
    
    tauri::Builder::default()
        .manage(game_manager)
        .invoke_handler(tauri::generate_handler![
            make_move,
            get_state,
            new_game,
            undo,
            set_theme,
            get_available_themes,
            get_stats,
            test_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
