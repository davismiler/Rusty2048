//! Shared components for Rusty2048
//! 
//! This module contains shared utilities, themes, and components
//! that can be used across different platforms.

use serde::{Deserialize, Serialize};

/// Color theme for the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub grid_background: String,
    pub tile_colors: Vec<String>,
    pub text_color: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Classic".to_string(),
            background: "#faf8ef".to_string(),
            grid_background: "#bbada0".to_string(),
            tile_colors: vec![
                "#cdc1b4".to_string(), // 0
                "#eee4da".to_string(), // 2
                "#ede0c8".to_string(), // 4
                "#f2b179".to_string(), // 8
                "#f59563".to_string(), // 16
                "#f67c5f".to_string(), // 32
                "#f65e3b".to_string(), // 64
                "#edcf72".to_string(), // 128
                "#edcc61".to_string(), // 256
                "#edc850".to_string(), // 512
                "#edc53f".to_string(), // 1024
                "#edc22e".to_string(), // 2048
            ],
            text_color: "#776e65".to_string(),
        }
    }
}

/// Game configuration that can be shared across platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedConfig {
    pub theme: Theme,
    pub board_size: usize,
    pub target_score: u32,
    pub enable_animations: bool,
    pub enable_sound: bool,
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            board_size: 4,
            target_score: 2048,
            enable_animations: true,
            enable_sound: false,
        }
    }
}

/// Animation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub duration_ms: u32,
    pub easing: String,
    pub enable_slide: bool,
    pub enable_merge: bool,
    pub enable_spawn: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration_ms: 150,
            easing: "ease-out".to_string(),
            enable_slide: true,
            enable_merge: true,
            enable_spawn: true,
        }
    }
}
