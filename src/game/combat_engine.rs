//! Enhanced Combat Engine - Event-driven, decoupled combat processing
//! 
//! This module replaces direct state mutation with event emission,
//! allowing the UI and game state to remain cleanly separated.

use std::time::Instant;
use std::sync::Arc;
use rand::Rng;

use super::combat_events::*;
use super::player::Player;
use super::enemy::Enemy;
use crate::data::GameData;

/// Pure combat state - no side effects, just data
#[derive(Debug, Clone)]
pub struct CombatEngine {
    // Enemy state
    pub enemy: Enemy,
    pub enemy_hp: i32,
    pub enemy_max_hp: i32,
    
    // Player combat state (not the actual player, just combat-relevant)
    pub player_shield: i32,
    
    // Typing state
    pub current_word: String,
    pub typed_input: String,
    pub char_correctness: Vec<bool>, // Track each character's correctness
    
    // Timing
    pub time_limit: f32,
    pub time_remaining: f32,
    pub last_update: Instant,
    pub typing_started: bool,
    
    // Combat progress
    pub turn: i32,
    pub phase: CombatPhaseType,
    pub combo: i32,
    pub words_completed: i32,
    
    // Difficulty & performance
    pub difficulty: DifficultyParams,
    pub performance: PerformanceTracker,
    
    // Data source
    pub game_data: Arc<GameData>,
    pub floor: u32,
    pub use_sentences: bool,
    
    // Event buffer - collected during update, drained by game loop
    events: Vec<CombatEvent>,
}

impl CombatEngine {
    pub fn new(enemy: Enemy, game_data: Arc<GameData>, floor: u32) -> Self {
        let difficulty = if enemy.is_boss {
            DifficultyParams::for_boss(floor as i32)
        } else {
            DifficultyParams::for_floor(floor as i32)
        };
        
        let use_sentences = enemy.is_boss || floor >= 5;
        let starting_word = if use_sentences {
            game_data.get_sentence(floor.min(10))
        } else {
            game_data.get_word(floor.min(10))
        };
        
        let time_limit = Self::calculate_time_limit(&starting_word, &difficulty);
        
        Self {
            enemy_hp: enemy.max_hp,
            enemy_max_hp: enemy.max_hp,
            enemy,
            player_shield: 0,
            current_word: starting_word,
            typed_input: String::new(),
            char_correctness: Vec::new(),
            time_limit,
            time_remaining: time_limit,
            last_update: Instant::now(),
            typing_started: false,
            turn: 1,
            phase: CombatPhaseType::PlayerTurn,
            combo: 0,
            words_completed: 0,
            difficulty,
            performance: PerformanceTracker::default(),
            game_data,
            floor,
            use_sentences,
            events: Vec::new(),
        }
    }
    
    /// Calculate time limit based on word length and difficulty
    fn calculate_time_limit(word: &str, difficulty: &DifficultyParams) -> f32 {
        let base_time = word.len() as f32 * difficulty.time_per_char;
        base_time.clamp(difficulty.min_time, difficulty.max_time)
    }
    
    /// Main update tick - call this every frame
    /// Returns events that occurred during this tick
    pub fn update(&mut self, delta_seconds: f32) -> Vec<CombatEvent> {
        self.events.clear();
        
        match self.phase {
            CombatPhaseType::PlayerTurn => {
                self.update_player_turn(delta_seconds);
            }
            CombatPhaseType::EnemyTurn => {
                // Enemy turn is instant, handled separately
            }
            _ => {}
        }
        
        std::mem::take(&mut self.events)
    }
    
    fn update_player_turn(&mut self, delta_seconds: f32) {
        if !self.typing_started {
            return;
        }
        
        self.time_remaining -= delta_seconds;
        
        if self.time_remaining <= 0.0 {
            self.time_remaining = 0.0;
            self.on_timeout();
        }
        
        // Check for enemy interrupt (advanced mechanic)
        if self.difficulty.enemy_can_interrupt && self.typed_input.len() > 0 {
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < self.difficulty.interrupt_chance * delta_seconds {
                self.on_interrupt();
            }
        }
    }
    
    /// Handle a character being typed
    pub fn on_char(&mut self, c: char) -> Vec<CombatEvent> {
        self.events.clear();
        
        if self.phase != CombatPhaseType::PlayerTurn {
            return std::mem::take(&mut self.events);
        }
        
        // Start timer on first keystroke
        if !self.typing_started {
            self.typing_started = true;
            self.last_update = Instant::now();
        }
        
        let pos = self.typed_input.len();
        let expected = self.current_word.chars().nth(pos);
        
        let correct = expected == Some(c);
        self.char_correctness.push(correct);
        self.typed_input.push(c);
        
        if !correct {
            self.performance.record_mistake();
            self.events.push(CombatEvent::CharTyped {
                correct: false,
                combo_maintained: false,
            });
        } else {
            self.events.push(CombatEvent::CharTyped {
                correct: true,
                combo_maintained: self.combo > 0,
            });
        }
        
        // Check word completion
        if self.typed_input.len() >= self.current_word.len() {
            self.on_word_complete();
        }
        
        std::mem::take(&mut self.events)
    }
    
    /// Handle backspace
    pub fn on_backspace(&mut self) -> Vec<CombatEvent> {
        self.events.clear();
        
        if self.phase != CombatPhaseType::PlayerTurn {
            return std::mem::take(&mut self.events);
        }
        
        if !self.typed_input.is_empty() {
            self.typed_input.pop();
            self.char_correctness.pop();
            self.performance.record_backspace();
        }
        
        std::mem::take(&mut self.events)
    }
    
    /// Called when a word is completed (correctly or not)
    fn on_word_complete(&mut self) {
        let wpm = self.calculate_wpm();
        let accuracy = self.calculate_word_accuracy();
        let perfect = self.typed_input == self.current_word && 
                      self.char_correctness.iter().all(|&c| c);
        
        self.performance.record_word(wpm, accuracy, perfect);
        self.words_completed += 1;
        
        if self.typed_input == self.current_word {
            // Correct word
            self.on_word_correct(wpm, accuracy, perfect);
        } else {
            // Mistyped word
            self.on_word_failed(FailReason::Mistyped);
        }
    }
    
    fn on_word_correct(&mut self, wpm: f32, accuracy: f32, perfect: bool) {
        // Calculate base damage
        let mut damage = self.calculate_base_damage();
        let mut events_to_add = Vec::new();
        
        // Combo bonus
        self.combo += 1;
        if self.combo > self.performance.best_combo {
            self.performance.best_combo = self.combo;
        }
        
        let combo_mult = 1.0 + (self.combo as f32 * self.difficulty.combo_damage_mult)
            .min(self.difficulty.max_combo_mult - 1.0);
        let combo_bonus = (damage as f32 * (combo_mult - 1.0)) as i32;
        damage = (damage as f32 * combo_mult) as i32;
        
        events_to_add.push(CombatEvent::ComboIncreased {
            new_combo: self.combo,
            bonus_damage: combo_bonus,
        });
        
        // Perfect word bonus
        if perfect {
            let perfect_bonus = (damage as f32 * (self.difficulty.perfect_mult - 1.0)) as i32;
            damage = (damage as f32 * self.difficulty.perfect_mult) as i32;
            events_to_add.push(CombatEvent::PerfectWordBonus {
                damage_mult: self.difficulty.perfect_mult,
            });
            
            // Check for perfect streak bonus
            if self.performance.perfect_streak >= 3 {
                events_to_add.push(CombatEvent::StreakBonus {
                    streak_type: StreakType::PerfectWords(self.performance.perfect_streak),
                    bonus: self.performance.perfect_streak * 5,
                });
            }
        }
        
        // Speed bonus
        if wpm >= self.difficulty.speed_bonus_wpm {
            let speed_mult = 1.0 + (wpm - self.difficulty.speed_bonus_wpm) / 100.0;
            damage = (damage as f32 * speed_mult) as i32;
            events_to_add.push(CombatEvent::SpeedBonus {
                wpm,
                damage_mult: speed_mult,
            });
        }
        
        // Accuracy penalty
        if accuracy < self.difficulty.accuracy_penalty_threshold {
            let penalty = 1.0 - ((self.difficulty.accuracy_penalty_threshold - accuracy) * 0.5);
            damage = (damage as f32 * penalty.max(0.5)) as i32;
            events_to_add.push(CombatEvent::AccuracyPenalty {
                accuracy,
                damage_reduction: 1.0 - penalty,
            });
        }
        
        // Apply damage
        let overkill = (damage - self.enemy_hp).max(0);
        self.enemy_hp = (self.enemy_hp - damage).max(0);
        
        self.events.push(CombatEvent::WordCompleted {
            word: self.current_word.clone(),
            wpm,
            accuracy,
            perfect,
        });
        
        self.events.push(CombatEvent::DamageDealt {
            amount: damage,
            critical: perfect,
            overkill,
        });
        
        self.events.extend(events_to_add);
        
        // Check for victory
        if self.enemy_hp <= 0 {
            self.on_victory();
        } else {
            self.transition_to_enemy_turn();
        }
    }
    
    fn on_word_failed(&mut self, reason: FailReason) {
        let was_combo = self.combo;
        self.combo = 0;
        self.performance.perfect_streak = 0;
        
        self.events.push(CombatEvent::WordFailed {
            word: self.current_word.clone(),
            typed: self.typed_input.clone(),
            reason,
        });
        
        if was_combo > 0 {
            self.events.push(CombatEvent::ComboLost { was_combo });
        }
        
        self.transition_to_enemy_turn();
    }
    
    fn on_timeout(&mut self) {
        self.on_word_failed(FailReason::Timeout);
    }
    
    fn on_interrupt(&mut self) {
        // Enemy interrupted mid-word
        self.events.push(CombatEvent::Message(
            format!("{} interrupts your typing!", self.enemy.name)
        ));
        self.on_word_failed(FailReason::Interrupted);
    }
    
    fn transition_to_enemy_turn(&mut self) {
        let old_phase = self.phase;
        self.phase = CombatPhaseType::EnemyTurn;
        self.events.push(CombatEvent::PhaseChanged {
            from: old_phase,
            to: self.phase,
        });
    }
    
    /// Execute enemy turn and return to player turn
    pub fn execute_enemy_turn(&mut self, player: &mut Player) -> Vec<CombatEvent> {
        self.events.clear();
        
        if self.phase != CombatPhaseType::EnemyTurn {
            return std::mem::take(&mut self.events);
        }
        
        let raw_damage = self.enemy.attack_power;
        let blocked = self.player_shield.min(raw_damage);
        self.player_shield = (self.player_shield - blocked).max(0);
        let actual_damage = raw_damage - blocked;
        
        player.take_damage(actual_damage);
        
        self.events.push(CombatEvent::DamageTaken {
            amount: actual_damage,
            blocked,
        });
        
        self.events.push(CombatEvent::Message(
            format!("{} {} for {} damage!", 
                self.enemy.name, 
                self.enemy.get_attack_message(),
                actual_damage)
        ));
        
        // Check for player defeat
        if player.hp <= 0 {
            self.on_defeat();
        } else {
            self.start_next_turn();
        }
        
        std::mem::take(&mut self.events)
    }
    
    fn start_next_turn(&mut self) {
        self.turn += 1;
        self.events.push(CombatEvent::TurnEnded { turn_number: self.turn - 1 });
        
        // Select new word, potentially scaling with performance
        let effective_difficulty = if self.performance.is_performing_well() {
            (self.floor + 1).min(10)
        } else if self.performance.is_struggling() {
            self.floor.saturating_sub(1).max(1)
        } else {
            self.floor
        };
        
        self.current_word = if self.use_sentences {
            self.game_data.get_sentence(effective_difficulty)
        } else {
            self.game_data.get_word(effective_difficulty)
        };
        
        self.typed_input.clear();
        self.char_correctness.clear();
        self.time_limit = Self::calculate_time_limit(&self.current_word, &self.difficulty);
        self.time_remaining = self.time_limit;
        self.typing_started = false;
        
        let old_phase = self.phase;
        self.phase = CombatPhaseType::PlayerTurn;
        self.events.push(CombatEvent::PhaseChanged {
            from: old_phase,
            to: self.phase,
        });
    }
    
    fn on_victory(&mut self) {
        let old_phase = self.phase;
        self.phase = CombatPhaseType::Victory;
        
        self.events.push(CombatEvent::EnemyDefeated {
            xp_reward: self.enemy.xp_reward,
            gold_reward: self.enemy.gold_reward,
        });
        
        self.events.push(CombatEvent::PhaseChanged {
            from: old_phase,
            to: self.phase,
        });
    }
    
    fn on_defeat(&mut self) {
        let old_phase = self.phase;
        self.phase = CombatPhaseType::Defeat;
        
        self.events.push(CombatEvent::PlayerDefeated);
        self.events.push(CombatEvent::PhaseChanged {
            from: old_phase,
            to: self.phase,
        });
    }
    
    fn calculate_base_damage(&self) -> i32 {
        // Base damage scales with word length
        let length_factor = (self.current_word.len() as f32 / 5.0).sqrt();
        (10.0 * length_factor) as i32
    }
    
    fn calculate_wpm(&self) -> f32 {
        if !self.typing_started || self.time_remaining >= self.time_limit {
            return 0.0;
        }
        
        let time_taken = self.time_limit - self.time_remaining;
        if time_taken <= 0.0 {
            return 0.0;
        }
        
        let chars = self.typed_input.len() as f32;
        let words = chars / 5.0; // Standard WPM calculation
        let minutes = time_taken / 60.0;
        
        words / minutes
    }
    
    fn calculate_word_accuracy(&self) -> f32 {
        if self.char_correctness.is_empty() {
            return 1.0;
        }
        
        let correct = self.char_correctness.iter().filter(|&&c| c).count();
        correct as f32 / self.char_correctness.len() as f32
    }
    
    /// Check if combat is over
    pub fn is_finished(&self) -> bool {
        matches!(self.phase, CombatPhaseType::Victory | CombatPhaseType::Defeat | CombatPhaseType::Fled)
    }
    
    /// Get current combat result if finished
    pub fn get_result(&self) -> Option<CombatResult> {
        if !self.is_finished() {
            return None;
        }
        
        Some(CombatResult {
            victory: self.phase == CombatPhaseType::Victory,
            xp_reward: if self.phase == CombatPhaseType::Victory { self.enemy.xp_reward } else { 0 },
            gold_reward: if self.phase == CombatPhaseType::Victory { self.enemy.gold_reward } else { 0 },
            turns_taken: self.turn,
            words_completed: self.words_completed,
            max_combo: self.performance.best_combo,
            accuracy: self.performance.average_accuracy(),
            peak_wpm: self.performance.peak_wpm,
            perfect_words: self.performance.total_perfect,
        })
    }
}

/// Result of a combat encounter
#[derive(Debug, Clone)]
pub struct CombatResult {
    pub victory: bool,
    pub xp_reward: i32,
    pub gold_reward: i32,
    pub turns_taken: i32,
    pub words_completed: i32,
    pub max_combo: i32,
    pub accuracy: f32,
    pub peak_wpm: f32,
    pub perfect_words: i32,
}
