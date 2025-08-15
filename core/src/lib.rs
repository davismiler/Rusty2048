//! Core game logic for Rusty2048
//!
//! This module provides the fundamental game mechanics including:
//! - Game board representation
//! - Move validation and execution
//! - Score calculation
//! - Game state management
//! - Random number generation with seed support

pub mod ai;
pub mod board;
pub mod error;
pub mod game;
pub mod replay;
pub mod rng;
pub mod score;
pub mod stats;

pub use ai::{AIAlgorithm, AIGameController, AIPlayer};
pub use board::Board;
pub use error::{GameError, GameResult};
pub use game::{Direction, Game, GameState};
pub use replay::{
    ReplayData, ReplayManager, ReplayMetadata, ReplayMove, ReplayPlayer, ReplayRecorder,
};
pub use rng::GameRng;
pub use score::Score;
pub use stats::{create_session_stats, GameSessionStats, StatisticsManager, StatisticsSummary};

/// Get current time as Unix timestamp
pub fn get_current_time() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        // For WASM, return 0 for now
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// Game configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameConfig {
    /// Board size (default: 4)
    pub board_size: usize,
    /// Target score to win (default: 2048)
    pub target_score: u32,
    /// Whether to allow undo (default: true)
    pub allow_undo: bool,
    /// Random seed for reproducible games
    pub seed: Option<u64>,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            board_size: 4,
            target_score: 2048,
            allow_undo: true,
            seed: None,
        }
    }
}

/// Game statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct GameStats {
    /// Current score
    pub score: u32,
    /// Best score achieved
    pub best_score: u32,
    /// Number of moves made
    pub moves: u32,
    /// Game duration in seconds
    pub duration: u64,
    /// Whether the game is won
    pub won: bool,
    /// Whether the game is over
    pub game_over: bool,
}
