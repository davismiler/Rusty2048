use wasm_bindgen::prelude::*;
use rusty2048_core::{Game, GameConfig, Direction, GameState};
use rusty2048_shared::{I18n, Language, TranslationKey};
use serde::{Serialize, Deserialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Rusty2048Web {
    game: Game,
    i18n: I18n,
}

#[wasm_bindgen]
impl Rusty2048Web {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        
        let config = GameConfig::default();
        let game = Game::new(config).expect("Failed to create game");
        let mut i18n = I18n::new();
        
        // Try to detect browser language
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();
            if let Some(lang) = navigator.language() {
                if let Some(language) = Language::from_code(&lang) {
                    i18n.set_language(language);
                }
            }
        }
        
        Self { game, i18n }
    }
    
    /// Get current language
    pub fn get_language(&self) -> String {
        self.i18n.current_language().code().to_string()
    }
    
    /// Set language
    pub fn set_language(&mut self, language_code: &str) -> Result<(), JsValue> {
        if let Some(language) = Language::from_code(language_code) {
            self.i18n.set_language(language);
            Ok(())
        } else {
            Err(JsValue::from_str("Invalid language code"))
        }
    }
    
    /// Get supported languages
    pub fn get_supported_languages(&self) -> JsValue {
        let languages: Vec<String> = self.i18n.supported_languages()
            .iter()
            .map(|lang| lang.code().to_string())
            .collect();
        serde_wasm_bindgen::to_value(&languages).unwrap()
    }
    
    /// Get translation for a key
    pub fn get_translation(&self, key: &str) -> String {
        // Convert string key to TranslationKey enum
        let translation_key = match key {
            "score" => TranslationKey::Score,
            "best" => TranslationKey::Best,
            "moves" => TranslationKey::Moves,
            "time" => TranslationKey::Time,
            "new_game" => TranslationKey::NewGame,
            "undo" => TranslationKey::Undo,
            "game_over" => TranslationKey::GameOver,
            "congratulations" => TranslationKey::Congratulations,
            "you_won" => TranslationKey::YouWon,
            "press_r_to_restart" => TranslationKey::PressRToRestart,
            "continue_playing" => TranslationKey::ContinuePlaying,
            "controls" => TranslationKey::Controls,
            "move_tiles" => TranslationKey::MoveTiles,
            "restart" => TranslationKey::Restart,
            "undo_move" => TranslationKey::UndoMove,
            "cycle_theme" => TranslationKey::CycleTheme,
            "select_theme" => TranslationKey::SelectTheme,
            "theme_help" => TranslationKey::ThemeHelp,
            "replay_mode" => TranslationKey::ReplayMode,
            "statistics_charts" => TranslationKey::StatisticsCharts,
            "ai_mode" => TranslationKey::AIMode,
            "help" => TranslationKey::Help,
            "quit" => TranslationKey::Quit,
            "language" => TranslationKey::Help, // Use Help as placeholder for "Language"
            _ => TranslationKey::Help, // Default fallback
        };
        
        self.i18n.t(&translation_key)
    }

    pub fn new_game(&mut self) -> Result<(), JsValue> {
        self.game.new_game().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn make_move(&mut self, direction: &str) -> Result<bool, JsValue> {
        let dir = match direction {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => return Err(JsValue::from_str("Invalid direction")),
        };
        
        self.game.make_move(dir).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn get_board(&self) -> Vec<u32> {
        let board = self.game.board();
        let mut result = Vec::new();
        for row in 0..board.size() {
            for col in 0..board.size() {
                if let Ok(tile) = board.get_tile(row, col) {
                    result.push(tile.value);
                }
            }
        }
        result
    }

    pub fn get_score(&self) -> JsValue {
        let score = self.game.score();
        serde_wasm_bindgen::to_value(&score).unwrap()
    }

    pub fn get_state(&self) -> String {
        match self.game.state() {
            GameState::Playing => "playing".to_string(),
            GameState::Won => "won".to_string(),
            GameState::GameOver => "game_over".to_string(),
        }
    }

    pub fn get_moves(&self) -> u32 {
        self.game.moves()
    }

    pub fn undo(&mut self) -> Result<(), JsValue> {
        self.game.undo().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), JsValue> {
        // Theme setting logic would go here
        Ok(())
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
