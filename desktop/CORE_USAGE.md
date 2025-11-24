# Using Core Logic in Desktop Version

This guide explains how to use the `rusty2048-core` library in the desktop version and how to add advanced features like AI mode, replay system, and statistics.

## Current Architecture

The desktop version uses a **GameManager** wrapper pattern:

```rust
struct GameManager {
    game: Game,           // Core game logic
    theme: Theme,         // UI theme
    i18n: I18n,          // Internationalization
}
```

The `Game` struct from `rusty2048-core` is wrapped in `Arc<Mutex<>>` for thread-safe access across Tauri commands.

## Basic Core Usage

### 1. Creating a Game

```rust
use rusty2048_core::{Game, GameConfig};

// Default configuration
let config = GameConfig::default();
let game = Game::new(config)?;

// Custom configuration
let config = GameConfig {
    board_size: 4,
    target_score: 2048,
    allow_undo: true,
    seed: Some(12345), // For reproducible games
};
let game = Game::new(config)?;
```

### 2. Making Moves

```rust
use rusty2048_core::Direction;

// Make a move
match game.make_move(Direction::Up) {
    Ok(moved) => {
        if moved {
            println!("Move successful!");
        } else {
            println!("No tiles moved");
        }
    }
    Err(e) => println!("Error: {}", e),
}
```

### 3. Getting Game State

```rust
// Get board
let board = game.board();
let size = board.size();
let tile = board.get_tile(row, col)?;

// Get score
let score = game.score();
let current = score.current();
let best = score.best();

// Get game state
let state = game.state(); // Playing, Won, or GameOver

// Get statistics
let stats = game.stats();
println!("Moves: {}, Duration: {}s", stats.moves, stats.duration);
```

### 4. Game Control

```rust
// Start new game
game.new_game()?;

// Undo last move
game.undo()?;

// Load from saved state
game.load_from_state(board_data, score, moves, game_state)?;
```

## Adding AI Mode

To add AI mode to the desktop version, follow this pattern:

### Step 1: Update GameManager

```rust
use rusty2048_core::{AIGameController, AIAlgorithm};

struct GameManager {
    game: Game,
    theme: Theme,
    i18n: I18n,
    ai_controller: Option<AIGameController>,  // Add AI controller
    ai_algorithm: AIAlgorithm,                // Current algorithm
}

impl GameManager {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // ... existing code ...
        Ok(GameManager {
            game,
            theme,
            i18n,
            ai_controller: None,
            ai_algorithm: AIAlgorithm::Greedy,
        })
    }

    fn enable_ai(&mut self, algorithm: AIAlgorithm) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.game.config().clone();
        let controller = AIGameController::new(config, algorithm)?;
        self.ai_controller = Some(controller);
        self.ai_algorithm = algorithm;
        Ok(())
    }

    fn disable_ai(&mut self) {
        self.ai_controller = None;
    }

    fn make_ai_move(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if let Some(controller) = &mut self.ai_controller {
            // Sync AI controller with current game state
            *controller.game_mut() = self.game.clone();
            
            // Make AI move
            let moved = controller.make_ai_move()?;
            
            if moved {
                // Update game with AI's move
                self.game = controller.game().clone();
            }
            
            Ok(moved)
        } else {
            Err("AI not enabled".into())
        }
    }

    fn switch_ai_algorithm(&mut self, algorithm: AIAlgorithm) -> Result<(), Box<dyn std::error::Error>> {
        self.enable_ai(algorithm)
    }
}
```

### Step 2: Add Tauri Commands

```rust
#[tauri::command]
async fn enable_ai(
    state: State<'_, Arc<Mutex<GameManager>>>,
    algorithm: String,
) -> Result<GameState, String> {
    let algo = match algorithm.as_str() {
        "greedy" => AIAlgorithm::Greedy,
        "expectimax" => AIAlgorithm::Expectimax,
        "mcts" => AIAlgorithm::MCTS,
        _ => return Err("Invalid algorithm".to_string()),
    };

    let mut mgr = state.lock().map_err(|_| "lock poisoned".to_string())?;
    mgr.enable_ai(algo).map_err(|e| e.to_string())?;
    Ok(mgr.get_state())
}

#[tauri::command]
async fn disable_ai(
    state: State<'_, Arc<Mutex<GameManager>>>,
) -> Result<GameState, String> {
    let mut mgr = state.lock().map_err(|_| "lock poisoned".to_string())?;
    mgr.disable_ai();
    Ok(mgr.get_state())
}

#[tauri::command]
async fn make_ai_move(
    state: State<'_, Arc<Mutex<GameManager>>>,
) -> Result<GameState, String> {
    let mut mgr = state.lock().map_err(|_| "lock poisoned".to_string())?;
    mgr.make_ai_move().map_err(|e| e.to_string())?;
    Ok(mgr.get_state())
}
```

### Step 3: Register Commands

```rust
tauri::Builder::default()
    .manage(game_manager)
    .invoke_handler(tauri::generate_handler![
        // ... existing commands ...
        enable_ai,
        disable_ai,
        make_ai_move,
    ])
    // ...
```

## Adding Replay System

### Step 1: Update GameManager

```rust
use rusty2048_core::{ReplayRecorder, ReplayPlayer, ReplayManager};

struct GameManager {
    game: Game,
    theme: Theme,
    i18n: I18n,
    replay_recorder: Option<ReplayRecorder>,  // For recording
    replay_player: Option<ReplayPlayer>,       // For playback
}

impl GameManager {
    fn start_recording(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.game.config().clone();
        let board = self.game.board().clone_board();
        let recorder = ReplayRecorder::new(config, board)?;
        self.replay_recorder = Some(recorder);
        Ok(())
    }

    fn stop_recording(&mut self) -> Result<ReplayData, Box<dyn std::error::Error>> {
        if let Some(recorder) = self.replay_recorder.take() {
            let replay_data = recorder.finish(
                self.game.score().current(),
                self.game.moves(),
                self.game.board().max_tile(),
            )?;
            Ok(replay_data)
        } else {
            Err("Not recording".into())
        }
    }

    fn record_move(&mut self, direction: Direction) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(recorder) = &mut self.replay_recorder {
            recorder.record_move(direction)?;
        }
        Ok(())
    }

    fn load_replay(&mut self, replay_data: ReplayData) -> Result<(), Box<dyn std::error::Error>> {
        let player = ReplayPlayer::new(replay_data)?;
        self.replay_player = Some(player);
        Ok(())
    }

    fn play_replay_step(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if let Some(player) = &mut self.replay_player {
            if let Some(move_data) = player.next_move() {
                // Apply the move to the game
                self.game.make_move(move_data.direction)?;
                Ok(true) // More moves available
            } else {
                Ok(false) // Replay finished
            }
        } else {
            Err("No replay loaded".into())
        }
    }
}
```

### Step 2: Add Tauri Commands

```rust
#[tauri::command]
async fn start_recording(
    state: State<'_, Arc<Mutex<GameManager>>>,
) -> Result<(), String> {
    let mut mgr = state.lock().map_err(|_| "lock poisoned".to_string())?;
    mgr.start_recording().map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_recording(
    state: State<'_, Arc<Mutex<GameManager>>>,
) -> Result<serde_json::Value, String> {
    let mut mgr = state.lock().map_err(|_| "lock poisoned".to_string())?;
    let replay_data = mgr.stop_recording().map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(&replay_data).unwrap())
}

#[tauri::command]
async fn play_replay_step(
    state: State<'_, Arc<Mutex<GameManager>>>,
) -> Result<GameState, String> {
    let mut mgr = state.lock().map_err(|_| "lock poisoned".to_string())?;
    let has_more = mgr.play_replay_step().map_err(|e| e.to_string())?;
    Ok(mgr.get_state())
}
```

## Adding Statistics

### Step 1: Update GameManager

```rust
use rusty2048_core::{StatisticsManager, create_session_stats};

struct GameManager {
    game: Game,
    theme: Theme,
    i18n: I18n,
    stats_manager: StatisticsManager,  // Add statistics manager
}

impl GameManager {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // ... existing code ...
        let stats_manager = StatisticsManager::new()?;
        Ok(GameManager {
            game,
            theme,
            i18n,
            stats_manager,
        })
    }

    fn record_game_session(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stats = create_session_stats(
            self.game.score().current(),
            self.game.moves(),
            self.game.stats().duration,
            self.game.board().max_tile(),
            self.game.state() == rusty2048_core::GameState::Won,
            self.game.stats().duration, // start_time
            rusty2048_core::get_current_time(), // end_time
        );
        self.stats_manager.record_session(stats)?;
        Ok(())
    }

    fn get_statistics_summary(&self) -> Result<StatisticsSummary, Box<dyn std::error::Error>> {
        Ok(self.stats_manager.get_summary()?)
    }
}
```

### Step 2: Add Tauri Commands

```rust
#[tauri::command]
async fn get_statistics(
    state: State<'_, Arc<Mutex<GameManager>>>,
) -> Result<serde_json::Value, String> {
    let mgr = state.lock().map_err(|_| "lock poisoned".to_string())?;
    let summary = mgr.get_statistics_summary().map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(&summary).unwrap())
}
```

## Complete Example: Enhanced GameManager

Here's a complete example combining all features:

```rust
use rusty2048_core::{
    Game, GameConfig, Direction, GameState,
    AIGameController, AIAlgorithm,
    ReplayRecorder, ReplayPlayer, ReplayData,
    StatisticsManager, create_session_stats,
};
use rusty2048_shared::{I18n, Theme};

struct GameManager {
    game: Game,
    theme: Theme,
    i18n: I18n,
    ai_controller: Option<AIGameController>,
    replay_recorder: Option<ReplayRecorder>,
    replay_player: Option<ReplayPlayer>,
    stats_manager: StatisticsManager,
}

impl GameManager {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = GameConfig::default();
        let game = Game::new(config)?;
        let theme = Theme::default();
        let i18n = I18n::new();
        let stats_manager = StatisticsManager::new()?;

        Ok(GameManager {
            game,
            theme,
            i18n,
            ai_controller: None,
            replay_recorder: None,
            replay_player: None,
            stats_manager,
        })
    }

    fn make_move(&mut self, direction: Direction) -> Result<bool, Box<dyn std::error::Error>> {
        // Record move if recording
        if let Some(recorder) = &mut self.replay_recorder {
            recorder.record_move(direction)?;
        }

        // Make the move
        self.game.make_move(direction)
            .map_err(|e| e.into())
    }

    fn get_state(&self) -> GameState {
        // Convert to your GameState struct
        // ... implementation ...
    }
}
```

## Best Practices

1. **Error Handling**: Always use `GameResult<T>` from core for error handling
2. **State Synchronization**: Keep AI controller in sync with game state
3. **Thread Safety**: Use `Arc<Mutex<>>` for shared state in Tauri
4. **Resource Management**: Clean up replay/AI resources when disabling features
5. **Serialization**: Use `serde` for data exchange between Rust and frontend

## Available Core Modules

- `Game`: Main game controller
- `Board`: Board representation and operations
- `Score`: Score tracking
- `AIGameController`: AI player with multiple algorithms
- `ReplayRecorder`/`ReplayPlayer`: Replay system
- `StatisticsManager`: Game statistics
- `GameConfig`: Game configuration
- `Direction`: Move directions
- `GameState`: Game state enum

## Frontend Integration

In your frontend (HTML/JS), call the Tauri commands:

```javascript
// Make a move
await invoke('make_move', { direction: 'up' });

// Enable AI
await invoke('enable_ai', { algorithm: 'greedy' });

// Make AI move
await invoke('make_ai_move');

// Get statistics
const stats = await invoke('get_statistics');
```

## Next Steps

1. Review the CLI implementation for reference patterns
2. Check `core/src/lib.rs` for all available exports
3. Look at `core/src/ai.rs` for AI implementation details
4. See `core/src/replay.rs` for replay system details
5. Examine `core/src/stats.rs` for statistics implementation

