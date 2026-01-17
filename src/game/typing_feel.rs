//! Typing Feel System - Juice, feedback, and satisfying keystrokes
//!
//! This module makes typing FEEL good. Every keystroke should have weight.
//! Visual feedback, combo systems, and moment-to-moment satisfaction.
//!
//! Design principles:
//! - Every correct keystroke should feel rewarding
//! - Errors should be clear but not punishing
//! - Combos build excitement
//! - Perfect words feel AMAZING
//! - Speed should feel powerful

use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Typing feedback state - tracks moment-to-moment feel
#[derive(Debug, Clone)]
pub struct TypingFeel {
    /// Current combo count
    pub combo: i32,
    /// Highest combo this session
    pub max_combo: i32,
    /// Combo multiplier for damage/rewards
    pub combo_multiplier: f32,
    /// Time since last keystroke (for rhythm detection)
    pub last_keystroke: Option<Instant>,
    /// Average time between keystrokes (for flow detection)
    pub keystroke_cadence: f32,
    /// Current flow state
    pub flow_state: FlowState,
    /// Pending visual effects
    pub pending_effects: Vec<TypingEffect>,
    /// Current accuracy (rolling average)
    pub accuracy: f32,
    /// Words per minute (rolling average)
    pub wpm: f32,
    /// Perfect words in a row
    pub perfect_streak: i32,
    /// Characters typed correctly in current word
    pub word_progress: i32,
    /// Total characters in current word
    pub word_total: i32,
    /// Screen shake intensity (0.0 - 1.0)
    pub screen_shake: f32,
    /// Color flash state
    pub color_flash: Option<ColorFlash>,
}

/// Flow state - how "in the zone" the player is
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlowState {
    /// Just starting, building momentum
    Building,
    /// Comfortable rhythm established
    Flowing,
    /// Peak performance, everything clicks
    Transcendent,
    /// Recovering from errors
    Recovering,
}

/// Visual effects that should trigger
#[derive(Debug, Clone)]
pub enum TypingEffect {
    /// Flash the correct character
    CharCorrect { char_index: usize },
    /// Flash the incorrect character
    CharIncorrect { char_index: usize, expected: char, got: char },
    /// Word completed successfully
    WordComplete { word: String, wpm: f32, accuracy: f32 },
    /// Word failed
    WordFailed { word: String, typed: String },
    /// Combo milestone reached
    ComboMilestone { combo: i32 },
    /// Perfect word bonus
    PerfectWord { word: String },
    /// Speed milestone
    SpeedMilestone { wpm: f32 },
    /// Flow state changed
    FlowChange { new_state: FlowState },
    /// Combo broken
    ComboBreak { was: i32 },
    /// Damage number
    DamageDealt { amount: i32, is_crit: bool },
    /// Screen shake trigger
    ScreenShake { intensity: f32, duration_ms: u32 },
    /// Text ripple effect
    TextRipple { center_index: usize, intensity: f32 },
}

/// Color flash for feedback
#[derive(Debug, Clone)]
pub struct ColorFlash {
    pub color: FlashColor,
    pub intensity: f32,
    pub duration_ms: u32,
    pub started: Instant,
}

#[derive(Debug, Clone, Copy)]
pub enum FlashColor {
    Green,   // Correct
    Red,     // Incorrect
    Gold,    // Perfect/Combo
    Blue,    // Flow state
    Purple,  // Critical/Special
}

impl Default for TypingFeel {
    fn default() -> Self {
        Self::new()
    }
}

impl TypingFeel {
    pub fn new() -> Self {
        Self {
            combo: 0,
            max_combo: 0,
            combo_multiplier: 1.0,
            last_keystroke: None,
            keystroke_cadence: 0.0,
            flow_state: FlowState::Building,
            pending_effects: Vec::new(),
            accuracy: 1.0,
            wpm: 0.0,
            perfect_streak: 0,
            word_progress: 0,
            word_total: 0,
            screen_shake: 0.0,
            color_flash: None,
        }
    }

    /// Reset for a new word
    pub fn start_word(&mut self, word_length: usize) {
        self.word_progress = 0;
        self.word_total = word_length as i32;
    }

    /// Called on each keystroke
    pub fn on_keystroke(&mut self, correct: bool, char_index: usize, expected: char, got: char) {
        let now = Instant::now();
        
        // Update cadence
        if let Some(last) = self.last_keystroke {
            let elapsed = now.duration_since(last).as_secs_f32();
            // Smooth the cadence
            self.keystroke_cadence = self.keystroke_cadence * 0.7 + elapsed * 0.3;
        }
        self.last_keystroke = Some(now);
        
        if correct {
            self.on_correct_keystroke(char_index);
        } else {
            self.on_incorrect_keystroke(char_index, expected, got);
        }
        
        self.update_flow_state();
    }

    fn on_correct_keystroke(&mut self, char_index: usize) {
        self.word_progress += 1;
        
        // Visual feedback
        self.pending_effects.push(TypingEffect::CharCorrect { char_index });
        
        // Subtle positive feedback
        self.color_flash = Some(ColorFlash {
            color: FlashColor::Green,
            intensity: 0.2,
            duration_ms: 50,
            started: Instant::now(),
        });
        
        // Text ripple on fast typing
        if self.keystroke_cadence < 0.15 && self.keystroke_cadence > 0.0 {
            self.pending_effects.push(TypingEffect::TextRipple {
                center_index: char_index,
                intensity: 0.3,
            });
        }
    }

    fn on_incorrect_keystroke(&mut self, char_index: usize, expected: char, got: char) {
        // Visual feedback
        self.pending_effects.push(TypingEffect::CharIncorrect { 
            char_index, 
            expected, 
            got 
        });
        
        // Error flash
        self.color_flash = Some(ColorFlash {
            color: FlashColor::Red,
            intensity: 0.5,
            duration_ms: 100,
            started: Instant::now(),
        });
        
        // Small screen shake on error
        self.screen_shake = 0.2;
        self.pending_effects.push(TypingEffect::ScreenShake {
            intensity: 0.2,
            duration_ms: 80,
        });
        
        // Reset perfect streak
        self.perfect_streak = 0;
    }

    /// Called when a word is completed
    pub fn on_word_complete(&mut self, word: &str, typed: &str, time_taken: f32) {
        let word_len = word.len();
        let correct_chars = word.chars()
            .zip(typed.chars())
            .filter(|(a, b)| a == b)
            .count();
        
        let accuracy = correct_chars as f32 / word_len as f32;
        let wpm = if time_taken > 0.0 {
            (word_len as f32 / 5.0) / (time_taken / 60.0)
        } else {
            0.0
        };
        
        // Update rolling averages
        self.accuracy = self.accuracy * 0.8 + accuracy * 0.2;
        self.wpm = self.wpm * 0.8 + wpm * 0.2;
        
        let is_perfect = word == typed;
        
        if is_perfect {
            self.on_perfect_word(word, wpm);
        } else {
            self.on_imperfect_word(word, typed);
        }
        
        self.pending_effects.push(TypingEffect::WordComplete {
            word: word.to_string(),
            wpm,
            accuracy,
        });
    }

    fn on_perfect_word(&mut self, word: &str, wpm: f32) {
        self.combo += 1;
        if self.combo > self.max_combo {
            self.max_combo = self.combo;
        }
        self.perfect_streak += 1;
        
        // Update combo multiplier
        self.combo_multiplier = 1.0 + (self.combo as f32 * 0.1).min(2.0);
        
        // Effects
        self.pending_effects.push(TypingEffect::PerfectWord {
            word: word.to_string(),
        });
        
        // Combo milestones
        if self.combo == 5 || self.combo == 10 || self.combo == 25 || self.combo == 50 || self.combo % 100 == 0 {
            self.pending_effects.push(TypingEffect::ComboMilestone { combo: self.combo });
            
            // Big feedback for milestones
            self.color_flash = Some(ColorFlash {
                color: FlashColor::Gold,
                intensity: 0.8,
                duration_ms: 200,
                started: Instant::now(),
            });
            
            self.screen_shake = 0.5;
            self.pending_effects.push(TypingEffect::ScreenShake {
                intensity: 0.5,
                duration_ms: 150,
            });
        }
        
        // Speed milestones
        if wpm >= 100.0 && self.wpm < 100.0 {
            self.pending_effects.push(TypingEffect::SpeedMilestone { wpm: 100.0 });
        } else if wpm >= 150.0 && self.wpm < 150.0 {
            self.pending_effects.push(TypingEffect::SpeedMilestone { wpm: 150.0 });
        }
    }

    fn on_imperfect_word(&mut self, word: &str, typed: &str) {
        if self.combo > 0 {
            self.pending_effects.push(TypingEffect::ComboBreak { was: self.combo });
        }
        self.combo = 0;
        self.combo_multiplier = 1.0;
        self.perfect_streak = 0;
        
        self.pending_effects.push(TypingEffect::WordFailed {
            word: word.to_string(),
            typed: typed.to_string(),
        });
    }

    fn update_flow_state(&mut self) {
        let old_state = self.flow_state;
        
        self.flow_state = if self.combo >= 20 && self.accuracy >= 0.95 && self.wpm >= 80.0 {
            FlowState::Transcendent
        } else if self.combo >= 5 && self.accuracy >= 0.85 {
            FlowState::Flowing
        } else if self.combo == 0 && self.accuracy < 0.7 {
            FlowState::Recovering
        } else {
            FlowState::Building
        };
        
        if self.flow_state != old_state {
            self.pending_effects.push(TypingEffect::FlowChange { new_state: self.flow_state });
            
            if self.flow_state == FlowState::Transcendent {
                self.color_flash = Some(ColorFlash {
                    color: FlashColor::Blue,
                    intensity: 0.6,
                    duration_ms: 300,
                    started: Instant::now(),
                });
            }
        }
    }

    /// Calculate damage with combo bonuses
    pub fn calculate_damage(&self, base_damage: i32) -> (i32, bool) {
        let mut damage = (base_damage as f32 * self.combo_multiplier) as i32;
        
        // Critical hit chance based on flow state
        let crit_chance = match self.flow_state {
            FlowState::Transcendent => 0.30,
            FlowState::Flowing => 0.15,
            FlowState::Building => 0.05,
            FlowState::Recovering => 0.02,
        };
        
        let mut rng = rand::thread_rng();
        let is_crit = rand::Rng::gen::<f32>(&mut rng) < crit_chance;
        
        if is_crit {
            damage = (damage as f32 * 1.5) as i32;
        }
        
        self.pending_effects.clone().into_iter().for_each(drop);
        
        (damage, is_crit)
    }

    /// Deal damage and show feedback
    pub fn deal_damage(&mut self, base_damage: i32) -> i32 {
        let (damage, is_crit) = self.calculate_damage(base_damage);
        
        self.pending_effects.push(TypingEffect::DamageDealt {
            amount: damage,
            is_crit,
        });
        
        if is_crit {
            self.color_flash = Some(ColorFlash {
                color: FlashColor::Purple,
                intensity: 0.7,
                duration_ms: 150,
                started: Instant::now(),
            });
            self.screen_shake = 0.6;
            self.pending_effects.push(TypingEffect::ScreenShake {
                intensity: 0.6,
                duration_ms: 120,
            });
        } else if damage > base_damage {
            self.screen_shake = 0.3;
            self.pending_effects.push(TypingEffect::ScreenShake {
                intensity: 0.3,
                duration_ms: 80,
            });
        }
        
        damage
    }

    /// Update effects (call each frame)
    pub fn tick(&mut self, delta: f32) {
        // Decay screen shake
        self.screen_shake = (self.screen_shake - delta * 5.0).max(0.0);
        
        // Check color flash expiry
        if let Some(ref flash) = self.color_flash {
            let elapsed = flash.started.elapsed().as_millis() as u32;
            if elapsed >= flash.duration_ms {
                self.color_flash = None;
            }
        }
    }

    /// Drain pending effects
    pub fn drain_effects(&mut self) -> Vec<TypingEffect> {
        std::mem::take(&mut self.pending_effects)
    }

    /// Get current flow description for UI
    pub fn flow_description(&self) -> &'static str {
        match self.flow_state {
            FlowState::Building => "Building momentum...",
            FlowState::Flowing => "In the flow!",
            FlowState::Transcendent => "TRANSCENDENT!",
            FlowState::Recovering => "Recovering...",
        }
    }

    /// Get combo description
    pub fn combo_description(&self) -> Option<String> {
        if self.combo >= 50 {
            Some(format!("{}x LEGENDARY!", self.combo))
        } else if self.combo >= 25 {
            Some(format!("{}x INCREDIBLE!", self.combo))
        } else if self.combo >= 10 {
            Some(format!("{}x AWESOME!", self.combo))
        } else if self.combo >= 5 {
            Some(format!("{}x combo!", self.combo))
        } else if self.combo > 0 {
            Some(format!("{}x", self.combo))
        } else {
            None
        }
    }
}

/// Rhythm detection for typing patterns
#[derive(Debug, Clone)]
pub struct RhythmDetector {
    /// Recent keystroke timings
    timings: Vec<f32>,
    /// Maximum timings to track
    max_timings: usize,
    /// Detected rhythm regularity (0.0 = erratic, 1.0 = perfect rhythm)
    pub regularity: f32,
    /// Average keystroke interval
    pub avg_interval: f32,
}

impl Default for RhythmDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl RhythmDetector {
    pub fn new() -> Self {
        Self {
            timings: Vec::new(),
            max_timings: 20,
            regularity: 0.0,
            avg_interval: 0.0,
        }
    }

    pub fn record(&mut self, interval: f32) {
        self.timings.push(interval);
        if self.timings.len() > self.max_timings {
            self.timings.remove(0);
        }
        self.analyze();
    }

    fn analyze(&mut self) {
        if self.timings.len() < 3 {
            return;
        }
        
        // Calculate average interval
        let sum: f32 = self.timings.iter().sum();
        self.avg_interval = sum / self.timings.len() as f32;
        
        // Calculate variance for regularity
        let variance: f32 = self.timings.iter()
            .map(|t| (t - self.avg_interval).powi(2))
            .sum::<f32>() / self.timings.len() as f32;
        
        let std_dev = variance.sqrt();
        
        // Regularity is inverse of coefficient of variation
        // (std_dev / mean), clamped and inverted
        let cv = std_dev / self.avg_interval.max(0.001);
        self.regularity = (1.0 - cv.min(1.0)).max(0.0);
    }

    pub fn is_rhythmic(&self) -> bool {
        self.regularity > 0.7
    }

    pub fn reset(&mut self) {
        self.timings.clear();
        self.regularity = 0.0;
        self.avg_interval = 0.0;
    }
}

/// Generates satisfying typing prompts
pub fn generate_satisfying_prompt(difficulty: u32, prefer_rhythmic: bool) -> String {
    // Words that are satisfying to type have:
    // - Alternating hands
    // - Rolling patterns
    // - Common bigrams
    
    let satisfying_words = [
        // Easy, rhythmic
        "the", "and", "that", "with", "this", "from", "they", "been",
        // Medium, flowing
        "world", "still", "should", "through", "while", "where", "think",
        // Harder but satisfying
        "keyboard", "typewriter", "flowing", "rhythm", "cascade", "whisper",
        // Very satisfying rolls
        "stewardess", "minimum", "humming", "opinion", "million",
    ];
    
    let rhythmic_sentences = [
        "The quick fox jumps.",
        "Words have weight here.",
        "Type with intention.",
        "Meaning flows through keys.",
        "Each stroke matters.",
    ];
    
    let complex_sentences = [
        "The corruption spreads through fractured syntax.",
        "Your fingers remember what your mind forgot.",
        "In the beginning was the word, and the word was power.",
        "Reality bends to the will of the accurate typist.",
        "The First Speaker's legacy lives in every keystroke.",
    ];
    
    if prefer_rhythmic && difficulty < 5 {
        rhythmic_sentences[rand::random::<usize>() % rhythmic_sentences.len()].to_string()
    } else if difficulty >= 7 {
        complex_sentences[rand::random::<usize>() % complex_sentences.len()].to_string()
    } else {
        // Construct from satisfying words
        let count = (difficulty as usize).clamp(2, 5);
        let mut words = Vec::new();
        for _ in 0..count {
            let idx = rand::random::<usize>() % satisfying_words.len();
            words.push(satisfying_words[idx]);
        }
        words.join(" ")
    }
}
