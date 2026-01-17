//! Combat Event System - Decoupled, event-driven combat architecture
//! 
//! This module provides a clean separation between combat logic and side effects.
//! Instead of directly mutating phases and player state, combat returns events
//! that the game loop processes.

use serde::{Deserialize, Serialize};

/// Events emitted by the combat system for the game loop to process
#[derive(Debug, Clone, PartialEq)]
pub enum CombatEvent {
    // Word/typing events
    WordCompleted {
        word: String,
        wpm: f32,
        accuracy: f32,
        perfect: bool,
    },
    WordFailed {
        word: String,
        typed: String,
        reason: FailReason,
    },
    CharTyped {
        correct: bool,
        combo_maintained: bool,
    },
    
    // Combat flow events
    DamageDealt {
        amount: i32,
        critical: bool,
        overkill: i32,
    },
    DamageTaken {
        amount: i32,
        blocked: i32,
    },
    EnemyDefeated {
        xp_reward: i32,
        gold_reward: i32,
    },
    PlayerDefeated,
    
    // Combo/streak events
    ComboIncreased {
        new_combo: i32,
        bonus_damage: i32,
    },
    ComboLost {
        was_combo: i32,
    },
    StreakBonus {
        streak_type: StreakType,
        bonus: i32,
    },
    
    // Special mechanics
    PerfectWordBonus {
        damage_mult: f32,
    },
    SpeedBonus {
        wpm: f32,
        damage_mult: f32,
    },
    AccuracyPenalty {
        accuracy: f32,
        damage_reduction: f32,
    },
    
    // Phase transitions
    PhaseChanged {
        from: CombatPhaseType,
        to: CombatPhaseType,
    },
    TurnEnded {
        turn_number: i32,
    },
    
    // UI feedback events
    Message(String),
    PlaySound(SoundEffect),
    ScreenShake(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FailReason {
    Timeout,
    Mistyped,
    Interrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreakType {
    PerfectWords(i32),     // N consecutive perfect words
    HighSpeed(i32),        // N consecutive high WPM words
    NoMistakes(i32),       // N words without any mistakes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatPhaseType {
    Intro,
    PlayerTurn,
    EnemyTurn,
    Victory,
    Defeat,
    Fled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoundEffect {
    KeyCorrect,
    KeyWrong,
    WordComplete,
    WordFailed,
    ComboUp,
    ComboLost,
    EnemyHit,
    PlayerHit,
    Victory,
    Defeat,
}

/// Difficulty parameters that scale with floor/enemy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyParams {
    /// Base time per character (seconds)
    pub time_per_char: f32,
    /// Minimum time limit
    pub min_time: f32,
    /// Maximum time limit
    pub max_time: f32,
    /// WPM threshold for speed bonus
    pub speed_bonus_wpm: f32,
    /// Accuracy threshold for penalty
    pub accuracy_penalty_threshold: f32,
    /// Combo damage multiplier per stack
    pub combo_damage_mult: f32,
    /// Maximum combo multiplier
    pub max_combo_mult: f32,
    /// Perfect word damage multiplier
    pub perfect_mult: f32,
    /// Whether enemy can interrupt mid-word
    pub enemy_can_interrupt: bool,
    /// Chance for enemy to interrupt (if enabled)
    pub interrupt_chance: f32,
}

impl Default for DifficultyParams {
    fn default() -> Self {
        Self {
            time_per_char: 0.3,
            min_time: 3.0,
            max_time: 20.0,
            speed_bonus_wpm: 60.0,
            accuracy_penalty_threshold: 0.8,
            combo_damage_mult: 0.1,
            max_combo_mult: 2.0,
            perfect_mult: 1.5,
            enemy_can_interrupt: false,
            interrupt_chance: 0.0,
        }
    }
}

impl DifficultyParams {
    /// Create difficulty params for a given floor
    pub fn for_floor(floor: i32) -> Self {
        let base = Self::default();
        let floor_factor = (floor as f32 - 1.0) * 0.1;
        
        Self {
            time_per_char: (base.time_per_char - floor_factor * 0.02).max(0.15),
            min_time: (base.min_time - floor_factor).max(2.0),
            max_time: base.max_time,
            speed_bonus_wpm: base.speed_bonus_wpm + floor_factor * 10.0,
            accuracy_penalty_threshold: base.accuracy_penalty_threshold + floor_factor * 0.05,
            combo_damage_mult: base.combo_damage_mult,
            max_combo_mult: base.max_combo_mult,
            perfect_mult: base.perfect_mult,
            enemy_can_interrupt: floor >= 5,
            interrupt_chance: if floor >= 5 { 0.1 + (floor - 5) as f32 * 0.05 } else { 0.0 },
        }
    }
    
    /// Create difficulty params for boss fights
    pub fn for_boss(floor: i32) -> Self {
        let mut params = Self::for_floor(floor);
        params.time_per_char *= 0.9; // Tighter timing
        params.enemy_can_interrupt = true;
        params.interrupt_chance = 0.15;
        params.perfect_mult = 2.0; // Higher reward for perfection
        params
    }
}

/// Typing performance tracker for analytics and difficulty scaling
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceTracker {
    /// Rolling average WPM over last N words
    pub recent_wpm: Vec<f32>,
    /// Rolling accuracy over last N words
    pub recent_accuracy: Vec<f32>,
    /// Consecutive perfect words
    pub perfect_streak: i32,
    /// Consecutive fast words (above threshold)
    pub speed_streak: i32,
    /// Total perfect words this combat
    pub total_perfect: i32,
    /// Total mistakes (wrong keys)
    pub total_mistakes: i32,
    /// Total backspaces used
    pub total_backspaces: i32,
    /// Highest WPM achieved
    pub peak_wpm: f32,
    /// Best combo achieved
    pub best_combo: i32,
}

impl PerformanceTracker {
    const ROLLING_WINDOW: usize = 5;
    
    pub fn record_word(&mut self, wpm: f32, accuracy: f32, perfect: bool) {
        // Update rolling averages
        self.recent_wpm.push(wpm);
        if self.recent_wpm.len() > Self::ROLLING_WINDOW {
            self.recent_wpm.remove(0);
        }
        
        self.recent_accuracy.push(accuracy);
        if self.recent_accuracy.len() > Self::ROLLING_WINDOW {
            self.recent_accuracy.remove(0);
        }
        
        // Update streaks
        if perfect {
            self.perfect_streak += 1;
            self.total_perfect += 1;
        } else {
            self.perfect_streak = 0;
        }
        
        // Update peak
        if wpm > self.peak_wpm {
            self.peak_wpm = wpm;
        }
    }
    
    pub fn record_mistake(&mut self) {
        self.total_mistakes += 1;
    }
    
    pub fn record_backspace(&mut self) {
        self.total_backspaces += 1;
    }
    
    pub fn average_wpm(&self) -> f32 {
        if self.recent_wpm.is_empty() {
            0.0
        } else {
            self.recent_wpm.iter().sum::<f32>() / self.recent_wpm.len() as f32
        }
    }
    
    pub fn average_accuracy(&self) -> f32 {
        if self.recent_accuracy.is_empty() {
            1.0
        } else {
            self.recent_accuracy.iter().sum::<f32>() / self.recent_accuracy.len() as f32
        }
    }
    
    /// Check if player is performing well (for adaptive difficulty)
    pub fn is_performing_well(&self) -> bool {
        self.average_wpm() > 50.0 && self.average_accuracy() > 0.95
    }
    
    /// Check if player is struggling (for mercy mechanics)
    pub fn is_struggling(&self) -> bool {
        self.average_wpm() < 25.0 || self.average_accuracy() < 0.7
    }
}
