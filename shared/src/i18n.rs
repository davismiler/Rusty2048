use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    /// Get language code
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
        }
    }

    /// Get language name
    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
        }
    }

    /// Get all supported languages
    pub fn all() -> Vec<Self> {
        vec![Language::English, Language::Chinese]
    }

    /// Parse from string
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" | "en-US" | "en-GB" => Some(Language::English),
            "zh" | "zh-CN" | "zh-TW" => Some(Language::Chinese),
            _ => None,
        }
    }
}

/// Translation keys
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranslationKey {
    // Game UI
    Title,
    Score,
    Best,
    Moves,
    Time,
    NewGame,
    Undo,
    GameOver,
    Congratulations,
    YouWon,
    PressRToRestart,
    ContinuePlaying,

    // Controls
    Controls,
    MoveTiles,
    Restart,
    UndoMove,
    CycleTheme,
    SelectTheme,
    ThemeHelp,
    ReplayMode,
    StatisticsCharts,
    AIMode,
    Help,
    Quit,

    // Replay Mode
    ReplayModeTitle,
    StartRecording,
    LoadReplay,
    ListReplays,
    BackToMenu,
    PlayPause,
    StepThrough,
    AdjustSpeed,
    StopRecording,

    // AI Mode
    AIModeTitle,
    ToggleAutoPlay,
    SwitchAlgorithm,
    AdjustSpeedAI,
    ExitImmediately,
    Greedy,
    Expectimax,
    MCTS,

    // Charts
    ChartsTitle,
    Summary,
    ScoreTrend,
    EfficiencyTrend,
    TileAchievements,
    RecentGames,
    NavigateCharts,
    ToggleCharts,

    // Statistics
    Statistics,
    GamesPlayed,
    GamesWon,
    WinRate,
    HighestScore,
    AverageScore,
    TotalMoves,
    AverageMoves,
    TotalPlayTime,
    AverageDuration,
    HighestTile,
    ScoreDistribution,
    LowScore,
    MediumScore,
    HighScore,
    VeryHighScore,
    NoDataAvailable,
    NoGamesPlayed,
    NoRecentGames,

    // Themes
    ThemeClassic,
    ThemeDark,
    ThemeNeon,
    ThemeRetro,
    ThemePastel,
    AvailableThemes,
    PressTToCycle,
    PressNumbersToSelect,

    // Messages
    Loading,
    Error,
    Success,
    Warning,
    Info,

    // Time formatting
    Hours,
    Minutes,
    Seconds,
}

/// Translation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationData {
    pub language: Language,
    pub translations: HashMap<String, String>,
}

impl TranslationData {
    /// Create English translations
    pub fn english() -> Self {
        let mut translations = HashMap::new();

        // Game UI
        translations.insert("title".to_string(), "Rusty2048".to_string());
        translations.insert("score".to_string(), "Score".to_string());
        translations.insert("best".to_string(), "Best".to_string());
        translations.insert("moves".to_string(), "Moves".to_string());
        translations.insert("time".to_string(), "Time".to_string());
        translations.insert("new_game".to_string(), "New Game".to_string());
        translations.insert("undo".to_string(), "Undo".to_string());
        translations.insert("game_over".to_string(), "Game Over!".to_string());
        translations.insert(
            "congratulations".to_string(),
            "🎉 Congratulations!".to_string(),
        );
        translations.insert("you_won".to_string(), "You won!".to_string());
        translations.insert(
            "press_r_to_restart".to_string(),
            "Press R to restart".to_string(),
        );
        translations.insert(
            "continue_playing".to_string(),
            "or continue playing".to_string(),
        );

        // Controls
        translations.insert("controls".to_string(), "Controls".to_string());
        translations.insert("move_tiles".to_string(), "WASD/Arrow Keys".to_string());
        translations.insert("restart".to_string(), "R".to_string());
        translations.insert("undo_move".to_string(), "U".to_string());
        translations.insert("cycle_theme".to_string(), "T".to_string());
        translations.insert("select_theme".to_string(), "1-5".to_string());
        translations.insert("theme_help".to_string(), "H".to_string());
        translations.insert("replay_mode".to_string(), "Replay".to_string());
        translations.insert("statistics_charts".to_string(), "Charts".to_string());
        translations.insert("ai_mode".to_string(), "AI".to_string());
        translations.insert("help".to_string(), "Help".to_string());
        translations.insert("quit".to_string(), "Quit".to_string());

        // Replay Mode
        translations.insert("replay_mode_title".to_string(), "Replay Mode".to_string());
        translations.insert("start_recording".to_string(), "Start Recording".to_string());
        translations.insert("load_replay".to_string(), "Load Replay".to_string());
        translations.insert("list_replays".to_string(), "List Replays".to_string());
        translations.insert("back_to_menu".to_string(), "Back to Menu".to_string());
        translations.insert("play_pause".to_string(), "Space".to_string());
        translations.insert("step_through".to_string(), "Left/Right".to_string());
        translations.insert("adjust_speed".to_string(), "+/-".to_string());
        translations.insert("stop_recording".to_string(), "S".to_string());

        // AI Mode
        translations.insert("ai_mode_title".to_string(), "AI Mode".to_string());
        translations.insert("toggle_auto_play".to_string(), "O".to_string());
        translations.insert("switch_algorithm".to_string(), "[ ]".to_string());
        translations.insert("adjust_speed_ai".to_string(), "+/-".to_string());
        translations.insert("exit_immediately".to_string(), "Q/ESC".to_string());
        translations.insert("greedy".to_string(), "Greedy".to_string());
        translations.insert("expectimax".to_string(), "Expectimax".to_string());
        translations.insert("mcts".to_string(), "MCTS".to_string());

        // Charts
        translations.insert("charts_title".to_string(), "Statistics Charts".to_string());
        translations.insert("summary".to_string(), "Summary".to_string());
        translations.insert("score_trend".to_string(), "Score Trend".to_string());
        translations.insert(
            "efficiency_trend".to_string(),
            "Efficiency Trend".to_string(),
        );
        translations.insert(
            "tile_achievements".to_string(),
            "Tile Achievements".to_string(),
        );
        translations.insert("recent_games".to_string(), "Recent Games".to_string());
        translations.insert("navigate_charts".to_string(), "Left/Right".to_string());
        translations.insert("toggle_charts".to_string(), "C".to_string());

        // Statistics
        translations.insert("statistics".to_string(), "Statistics".to_string());
        translations.insert("games_played".to_string(), "Games Played".to_string());
        translations.insert("games_won".to_string(), "Won".to_string());
        translations.insert("win_rate".to_string(), "Win Rate".to_string());
        translations.insert("highest_score".to_string(), "Highest Score".to_string());
        translations.insert("average_score".to_string(), "Average Score".to_string());
        translations.insert("total_moves".to_string(), "Total Moves".to_string());
        translations.insert("average_moves".to_string(), "Avg Moves".to_string());
        translations.insert("total_play_time".to_string(), "Total Play Time".to_string());
        translations.insert("average_duration".to_string(), "Avg Duration".to_string());
        translations.insert("highest_tile".to_string(), "Highest Tile".to_string());
        translations.insert(
            "score_distribution".to_string(),
            "Score Distribution".to_string(),
        );
        translations.insert("low_score".to_string(), "0-1000".to_string());
        translations.insert("medium_score".to_string(), "1001-5000".to_string());
        translations.insert("high_score".to_string(), "5001-10000".to_string());
        translations.insert("very_high_score".to_string(), "10001+".to_string());
        translations.insert(
            "no_data_available".to_string(),
            "No data available".to_string(),
        );
        translations.insert(
            "no_games_played".to_string(),
            "No games played yet!".to_string(),
        );
        translations.insert("no_recent_games".to_string(), "No recent games".to_string());

        // Themes
        translations.insert("theme_classic".to_string(), "Classic".to_string());
        translations.insert("theme_dark".to_string(), "Dark".to_string());
        translations.insert("theme_neon".to_string(), "Neon".to_string());
        translations.insert("theme_retro".to_string(), "Retro".to_string());
        translations.insert("theme_pastel".to_string(), "Pastel".to_string());
        translations.insert(
            "available_themes".to_string(),
            "Available Themes".to_string(),
        );
        translations.insert(
            "press_t_to_cycle".to_string(),
            "Press T to cycle themes".to_string(),
        );
        translations.insert(
            "press_numbers_to_select".to_string(),
            "or number keys 1-5 to select directly".to_string(),
        );

        // Messages
        translations.insert("loading".to_string(), "Loading...".to_string());
        translations.insert("error".to_string(), "Error".to_string());
        translations.insert("success".to_string(), "Success".to_string());
        translations.insert("warning".to_string(), "Warning".to_string());
        translations.insert("info".to_string(), "Info".to_string());

        // Time formatting
        translations.insert("hours".to_string(), "h".to_string());
        translations.insert("minutes".to_string(), "m".to_string());
        translations.insert("seconds".to_string(), "s".to_string());

        Self {
            language: Language::English,
            translations,
        }
    }

    /// Create Chinese translations
    pub fn chinese() -> Self {
        let mut translations = HashMap::new();

        // Game UI
        translations.insert("title".to_string(), "Rusty2048".to_string());
        translations.insert("score".to_string(), "分数".to_string());
        translations.insert("best".to_string(), "最高分".to_string());
        translations.insert("moves".to_string(), "步数".to_string());
        translations.insert("time".to_string(), "时间".to_string());
        translations.insert("new_game".to_string(), "新游戏".to_string());
        translations.insert("undo".to_string(), "撤销".to_string());
        translations.insert("game_over".to_string(), "游戏结束！".to_string());
        translations.insert("congratulations".to_string(), "🎉 恭喜！".to_string());
        translations.insert("you_won".to_string(), "你赢了！".to_string());
        translations.insert("press_r_to_restart".to_string(), "按R重新开始".to_string());
        translations.insert("continue_playing".to_string(), "或继续游戏".to_string());

        // Controls
        translations.insert("controls".to_string(), "控制".to_string());
        translations.insert("move_tiles".to_string(), "WASD/方向键".to_string());
        translations.insert("restart".to_string(), "R".to_string());
        translations.insert("undo_move".to_string(), "U".to_string());
        translations.insert("cycle_theme".to_string(), "T".to_string());
        translations.insert("select_theme".to_string(), "1-5".to_string());
        translations.insert("theme_help".to_string(), "H".to_string());
        translations.insert("replay_mode".to_string(), "回放".to_string());
        translations.insert("statistics_charts".to_string(), "图表".to_string());
        translations.insert("ai_mode".to_string(), "AI".to_string());
        translations.insert("help".to_string(), "帮助".to_string());
        translations.insert("quit".to_string(), "退出".to_string());

        // Replay Mode
        translations.insert("replay_mode_title".to_string(), "回放模式".to_string());
        translations.insert("start_recording".to_string(), "开始录制".to_string());
        translations.insert("load_replay".to_string(), "加载回放".to_string());
        translations.insert("list_replays".to_string(), "回放列表".to_string());
        translations.insert("back_to_menu".to_string(), "返回菜单".to_string());
        translations.insert("play_pause".to_string(), "空格".to_string());
        translations.insert("step_through".to_string(), "左右键".to_string());
        translations.insert("adjust_speed".to_string(), "+/-".to_string());
        translations.insert("stop_recording".to_string(), "S".to_string());

        // AI Mode
        translations.insert("ai_mode_title".to_string(), "AI模式".to_string());
        translations.insert("toggle_auto_play".to_string(), "O".to_string());
        translations.insert("switch_algorithm".to_string(), "[ ]".to_string());
        translations.insert("adjust_speed_ai".to_string(), "+/-".to_string());
        translations.insert("exit_immediately".to_string(), "Q/ESC".to_string());
        translations.insert("greedy".to_string(), "贪心".to_string());
        translations.insert("expectimax".to_string(), "期望最大化".to_string());
        translations.insert("mcts".to_string(), "蒙特卡洛".to_string());

        // Charts
        translations.insert("charts_title".to_string(), "统计图表".to_string());
        translations.insert("summary".to_string(), "摘要".to_string());
        translations.insert("score_trend".to_string(), "分数趋势".to_string());
        translations.insert("efficiency_trend".to_string(), "效率趋势".to_string());
        translations.insert("tile_achievements".to_string(), "瓦片成就".to_string());
        translations.insert("recent_games".to_string(), "最近游戏".to_string());
        translations.insert("navigate_charts".to_string(), "左右键".to_string());
        translations.insert("toggle_charts".to_string(), "C".to_string());

        // Statistics
        translations.insert("statistics".to_string(), "统计".to_string());
        translations.insert("games_played".to_string(), "游戏局数".to_string());
        translations.insert("games_won".to_string(), "胜利".to_string());
        translations.insert("win_rate".to_string(), "胜率".to_string());
        translations.insert("highest_score".to_string(), "最高分".to_string());
        translations.insert("average_score".to_string(), "平均分".to_string());
        translations.insert("total_moves".to_string(), "总步数".to_string());
        translations.insert("average_moves".to_string(), "平均步数".to_string());
        translations.insert("total_play_time".to_string(), "总游戏时间".to_string());
        translations.insert("average_duration".to_string(), "平均时长".to_string());
        translations.insert("highest_tile".to_string(), "最高瓦片".to_string());
        translations.insert("score_distribution".to_string(), "分数分布".to_string());
        translations.insert("low_score".to_string(), "0-1000".to_string());
        translations.insert("medium_score".to_string(), "1001-5000".to_string());
        translations.insert("high_score".to_string(), "5001-10000".to_string());
        translations.insert("very_high_score".to_string(), "10001+".to_string());
        translations.insert("no_data_available".to_string(), "暂无数据".to_string());
        translations.insert(
            "no_games_played".to_string(),
            "还没有玩过游戏！".to_string(),
        );
        translations.insert("no_recent_games".to_string(), "没有最近游戏".to_string());

        // Themes
        translations.insert("theme_classic".to_string(), "经典".to_string());
        translations.insert("theme_dark".to_string(), "暗黑".to_string());
        translations.insert("theme_neon".to_string(), "霓虹".to_string());
        translations.insert("theme_retro".to_string(), "复古".to_string());
        translations.insert("theme_pastel".to_string(), "粉彩".to_string());
        translations.insert("available_themes".to_string(), "可用主题".to_string());
        translations.insert(
            "press_t_to_cycle".to_string(),
            "按T循环切换主题".to_string(),
        );
        translations.insert(
            "press_numbers_to_select".to_string(),
            "或按数字键1-5直接选择".to_string(),
        );

        // Messages
        translations.insert("loading".to_string(), "加载中...".to_string());
        translations.insert("error".to_string(), "错误".to_string());
        translations.insert("success".to_string(), "成功".to_string());
        translations.insert("warning".to_string(), "警告".to_string());
        translations.insert("info".to_string(), "信息".to_string());

        // Time formatting
        translations.insert("hours".to_string(), "时".to_string());
        translations.insert("minutes".to_string(), "分".to_string());
        translations.insert("seconds".to_string(), "秒".to_string());

        Self {
            language: Language::Chinese,
            translations,
        }
    }
}

/// Internationalization manager
#[derive(Debug, Clone)]
pub struct I18n {
    current_language: Language,
    translations: HashMap<Language, TranslationData>,
}

impl I18n {
    /// Create a new I18n instance
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        translations.insert(Language::English, TranslationData::english());
        translations.insert(Language::Chinese, TranslationData::chinese());

        Self {
            current_language: Language::English,
            translations,
        }
    }

    /// Set current language
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    /// Get current language
    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// Get translation for a key
    pub fn t(&self, key: &TranslationKey) -> String {
        let key_str = self.key_to_string(key);
        if let Some(translation_data) = self.translations.get(&self.current_language) {
            if let Some(translation) = translation_data.translations.get(&key_str) {
                return translation.clone();
            }
        }

        // Fallback to English
        if let Some(translation_data) = self.translations.get(&Language::English) {
            if let Some(translation) = translation_data.translations.get(&key_str) {
                return translation.clone();
            }
        }

        // Return key as fallback
        key_str
    }

    /// Get translation with parameters
    pub fn t_with_params(&self, key: &TranslationKey, params: &[(&str, &str)]) -> String {
        let mut text = self.t(key);
        for (param, value) in params {
            text = text.replace(&format!("{{{}}}", param), value);
        }
        text
    }

    /// Convert translation key to string
    fn key_to_string(&self, key: &TranslationKey) -> String {
        match key {
            TranslationKey::Title => "title".to_string(),
            TranslationKey::Score => "score".to_string(),
            TranslationKey::Best => "best".to_string(),
            TranslationKey::Moves => "moves".to_string(),
            TranslationKey::Time => "time".to_string(),
            TranslationKey::NewGame => "new_game".to_string(),
            TranslationKey::Undo => "undo".to_string(),
            TranslationKey::GameOver => "game_over".to_string(),
            TranslationKey::Congratulations => "congratulations".to_string(),
            TranslationKey::YouWon => "you_won".to_string(),
            TranslationKey::PressRToRestart => "press_r_to_restart".to_string(),
            TranslationKey::ContinuePlaying => "continue_playing".to_string(),
            TranslationKey::Controls => "controls".to_string(),
            TranslationKey::MoveTiles => "move_tiles".to_string(),
            TranslationKey::Restart => "restart".to_string(),
            TranslationKey::UndoMove => "undo_move".to_string(),
            TranslationKey::CycleTheme => "cycle_theme".to_string(),
            TranslationKey::SelectTheme => "select_theme".to_string(),
            TranslationKey::ThemeHelp => "theme_help".to_string(),
            TranslationKey::ReplayMode => "replay_mode".to_string(),
            TranslationKey::StatisticsCharts => "statistics_charts".to_string(),
            TranslationKey::AIMode => "ai_mode".to_string(),
            TranslationKey::Help => "help".to_string(),
            TranslationKey::Quit => "quit".to_string(),
            TranslationKey::ReplayModeTitle => "replay_mode_title".to_string(),
            TranslationKey::StartRecording => "start_recording".to_string(),
            TranslationKey::LoadReplay => "load_replay".to_string(),
            TranslationKey::ListReplays => "list_replays".to_string(),
            TranslationKey::BackToMenu => "back_to_menu".to_string(),
            TranslationKey::PlayPause => "play_pause".to_string(),
            TranslationKey::StepThrough => "step_through".to_string(),
            TranslationKey::AdjustSpeed => "adjust_speed".to_string(),
            TranslationKey::StopRecording => "stop_recording".to_string(),
            TranslationKey::AIModeTitle => "ai_mode_title".to_string(),
            TranslationKey::ToggleAutoPlay => "toggle_auto_play".to_string(),
            TranslationKey::SwitchAlgorithm => "switch_algorithm".to_string(),
            TranslationKey::AdjustSpeedAI => "adjust_speed_ai".to_string(),
            TranslationKey::ExitImmediately => "exit_immediately".to_string(),
            TranslationKey::Greedy => "greedy".to_string(),
            TranslationKey::Expectimax => "expectimax".to_string(),
            TranslationKey::MCTS => "mcts".to_string(),
            TranslationKey::ChartsTitle => "charts_title".to_string(),
            TranslationKey::Summary => "summary".to_string(),
            TranslationKey::ScoreTrend => "score_trend".to_string(),
            TranslationKey::EfficiencyTrend => "efficiency_trend".to_string(),
            TranslationKey::TileAchievements => "tile_achievements".to_string(),
            TranslationKey::RecentGames => "recent_games".to_string(),
            TranslationKey::NavigateCharts => "navigate_charts".to_string(),
            TranslationKey::ToggleCharts => "toggle_charts".to_string(),
            TranslationKey::Statistics => "statistics".to_string(),
            TranslationKey::GamesPlayed => "games_played".to_string(),
            TranslationKey::GamesWon => "games_won".to_string(),
            TranslationKey::WinRate => "win_rate".to_string(),
            TranslationKey::HighestScore => "highest_score".to_string(),
            TranslationKey::AverageScore => "average_score".to_string(),
            TranslationKey::TotalMoves => "total_moves".to_string(),
            TranslationKey::AverageMoves => "average_moves".to_string(),
            TranslationKey::TotalPlayTime => "total_play_time".to_string(),
            TranslationKey::AverageDuration => "average_duration".to_string(),
            TranslationKey::HighestTile => "highest_tile".to_string(),
            TranslationKey::ScoreDistribution => "score_distribution".to_string(),
            TranslationKey::LowScore => "low_score".to_string(),
            TranslationKey::MediumScore => "medium_score".to_string(),
            TranslationKey::HighScore => "high_score".to_string(),
            TranslationKey::VeryHighScore => "very_high_score".to_string(),
            TranslationKey::NoDataAvailable => "no_data_available".to_string(),
            TranslationKey::NoGamesPlayed => "no_games_played".to_string(),
            TranslationKey::NoRecentGames => "no_recent_games".to_string(),
            TranslationKey::ThemeClassic => "theme_classic".to_string(),
            TranslationKey::ThemeDark => "theme_dark".to_string(),
            TranslationKey::ThemeNeon => "theme_neon".to_string(),
            TranslationKey::ThemeRetro => "theme_retro".to_string(),
            TranslationKey::ThemePastel => "theme_pastel".to_string(),
            TranslationKey::AvailableThemes => "available_themes".to_string(),
            TranslationKey::PressTToCycle => "press_t_to_cycle".to_string(),
            TranslationKey::PressNumbersToSelect => "press_numbers_to_select".to_string(),
            TranslationKey::Loading => "loading".to_string(),
            TranslationKey::Error => "error".to_string(),
            TranslationKey::Success => "success".to_string(),
            TranslationKey::Warning => "warning".to_string(),
            TranslationKey::Info => "info".to_string(),
            TranslationKey::Hours => "hours".to_string(),
            TranslationKey::Minutes => "minutes".to_string(),
            TranslationKey::Seconds => "seconds".to_string(),
        }
    }

    /// Get all supported languages
    pub fn supported_languages(&self) -> Vec<Language> {
        Language::all()
    }

    /// Format duration in localized format
    pub fn format_duration(&self, seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        if hours > 0 {
            format!(
                "{}{}{:02}{}{:02}{}",
                hours,
                self.t(&TranslationKey::Hours),
                minutes,
                self.t(&TranslationKey::Minutes),
                secs,
                self.t(&TranslationKey::Seconds)
            )
        } else {
            format!(
                "{}{}{:02}{}",
                minutes,
                self.t(&TranslationKey::Minutes),
                secs,
                self.t(&TranslationKey::Seconds)
            )
        }
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}
