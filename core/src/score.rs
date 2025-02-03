use serde::{Deserialize, Serialize};

/// Score tracking and calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    /// Current score
    current: u32,
    /// Best score achieved
    best: u32,
    /// Score gained from last move
    last_move: u32,
}

impl Score {
    /// Create a new score tracker
    pub fn new() -> Self {
        Self {
            current: 0,
            best: 0,
            last_move: 0,
        }
    }
    
    /// Get current score
    pub fn current(&self) -> u32 {
        self.current
    }
    
    /// Get best score
    pub fn best(&self) -> u32 {
        self.best
    }
    
    /// Get score from last move
    pub fn last_move(&self) -> u32 {
        self.last_move
    }
    
    /// Add points from a merge
    pub fn add_merge_points(&mut self, merged_value: u32) {
        self.last_move = merged_value;
        self.current += merged_value;
        
        if self.current > self.best {
            self.best = self.current;
        }
    }
    
    /// Reset current score (for new game)
    pub fn reset_current(&mut self) {
        self.current = 0;
        self.last_move = 0;
    }
    
    /// Reset all scores
    pub fn reset_all(&mut self) {
        self.current = 0;
        self.best = 0;
        self.last_move = 0;
    }
    
    /// Calculate score for a specific merge
    pub fn calculate_merge_score(merged_value: u32) -> u32 {
        merged_value
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_score_creation() {
        let score = Score::new();
        assert_eq!(score.current(), 0);
        assert_eq!(score.best(), 0);
        assert_eq!(score.last_move(), 0);
    }
    
    #[test]
    fn test_score_operations() {
        let mut score = Score::new();
        
        // Add merge points
        score.add_merge_points(4);
        assert_eq!(score.current(), 4);
        assert_eq!(score.best(), 4);
        assert_eq!(score.last_move(), 4);
        
        // Add more points
        score.add_merge_points(8);
        assert_eq!(score.current(), 12);
        assert_eq!(score.best(), 12);
        assert_eq!(score.last_move(), 8);
        
        // Reset current
        score.reset_current();
        assert_eq!(score.current(), 0);
        assert_eq!(score.best(), 12); // Best should remain
        assert_eq!(score.last_move(), 0);
    }
    
    #[test]
    fn test_merge_score_calculation() {
        assert_eq!(Score::calculate_merge_score(2), 2);
        assert_eq!(Score::calculate_merge_score(4), 4);
        assert_eq!(Score::calculate_merge_score(8), 8);
        assert_eq!(Score::calculate_merge_score(2048), 2048);
    }
}
