//! Combat system - the core typing battle experience!

use std::time::{Duration, Instant};
use std::sync::Arc;
use super::{player::Player, enemy::Enemy, spells::Spell};
use super::narrative_seed::TypingModifier;
use crate::data::GameData;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct CombatState {
    pub enemy: Enemy,
    pub turn: i32,
    pub player_shield: i32,
    pub combo: i32,
    pub max_combo: i32,
    pub words_typed: i32,
    pub words_correct: i32,
    pub total_chars: i32,
    pub correct_chars: i32,
    pub current_word: String,
    pub typed_input: String,
    pub time_limit: f32,
    pub time_remaining: f32,
    pub last_tick: Instant,
    pub battle_log: Vec<String>,
    pub phase: CombatPhase,
    pub result: Option<CombatResult>,
    pub typing_started: bool,
    pub game_data: Arc<GameData>,
    pub difficulty: u32,
    pub use_sentences: bool,
    pub floor: u32,
    /// Whether player is in spell casting mode
    pub spell_mode: bool,
    /// Currently selected spell index
    pub selected_spell: Option<usize>,
    /// The spell incantation to type (when in spell mode)
    pub spell_incantation: Option<String>,
    /// Active corruption modifier affecting typing
    pub corruption_modifier: Option<TypingModifier>,
    /// Damage from corruption mistakes this combat
    pub corruption_damage_taken: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatPhase {
    Intro,           // Enemy appeared!
    PlayerTurn,      // Player is typing
    EnemyTurn,       // Enemy attacks
    Victory,         // Player won
    Defeat,          // Player lost
    Fled,            // Player escaped
    Spared,          // Undertale-style spare
}

#[derive(Debug, Clone)]
pub struct CombatResult {
    pub victory: bool,
    pub fled: bool,
    pub spared: bool,
    pub xp_gained: i32,
    pub gold_gained: i32,
    pub turns_taken: i32,
    pub max_combo: i32,
    pub accuracy: f32,
    pub avg_wpm: f32,
}

impl CombatState {
    pub fn new(enemy: Enemy, game_data: Arc<GameData>, difficulty: u32, floor: u32, corruption: Option<TypingModifier>) -> Self {
        // Use sentences for bosses or high difficulty, otherwise words
        let use_sentences = enemy.is_boss || difficulty >= 5;
        let starting_word = if use_sentences {
            game_data.get_lore_sentence(floor, enemy.is_boss, Some(&enemy.name))
        } else {
            game_data.get_lore_word(floor, Some(&enemy.typing_theme))
        };
        
        // Adjust time limit based on content length
        let time_limit = if use_sentences {
            15.0 + (starting_word.len() as f32 * 0.1)
        } else {
            5.0 + (starting_word.len() as f32 * 0.2)
        };
        
        Self {
            enemy,
            turn: 1,
            player_shield: 0,
            combo: 0,
            max_combo: 0,
            words_typed: 0,
            words_correct: 0,
            total_chars: 0,
            correct_chars: 0,
            current_word: starting_word,
            typed_input: String::new(),
            time_limit,
            time_remaining: time_limit,
            last_tick: Instant::now(),
            battle_log: vec!["Type to attack!".to_string()],
            phase: CombatPhase::PlayerTurn,
            result: None,
            typing_started: false,
            game_data,
            difficulty,
            use_sentences,
            floor,
            spell_mode: false,
            selected_spell: None,
            spell_incantation: None,
            corruption_modifier: corruption,
            corruption_damage_taken: 0,
        }
    }

    pub fn start_turn(&mut self, word_pool: &[String]) {
        self.phase = CombatPhase::PlayerTurn;
        self.current_word = self.select_word(word_pool);
        self.typed_input.clear();
        self.time_remaining = self.time_limit;
        self.last_tick = Instant::now();
        self.typing_started = false;
    }

    fn select_word(&self, word_pool: &[String]) -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..word_pool.len());
        word_pool[index].clone()
    }

    pub fn tick(&mut self) {
        if self.phase != CombatPhase::PlayerTurn {
            return;
        }
        
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.last_tick = now;
        
        if self.typing_started {
            self.time_remaining -= elapsed.as_secs_f32();
            
            if self.time_remaining <= 0.0 {
                self.time_remaining = 0.0;
                self.on_word_timeout();
            }
        }
    }

    pub fn on_char_typed(&mut self, c: char) {
        if self.phase != CombatPhase::PlayerTurn {
            return;
        }

        if !self.typing_started {
            self.typing_started = true;
            self.last_tick = Instant::now();
        }

        self.typed_input.push(c);
        self.total_chars += 1;

        let expected_char = self.current_word.chars().nth(self.typed_input.len() - 1);
        if expected_char == Some(c) {
            self.correct_chars += 1;
        } else {
            // Corruption effect: MistakesDealDamage
            if let Some(TypingModifier::MistakesDealDamage { damage_per_error }) = &self.corruption_modifier {
                self.corruption_damage_taken += damage_per_error;
                self.battle_log.push(format!("Corruption punishes your error! (-{} HP)", damage_per_error));
            }
        }

        // Check if word is complete
        if self.typed_input.len() >= self.current_word.len() {
            self.on_word_complete();
        }
    }

    pub fn on_backspace(&mut self) {
        if self.phase != CombatPhase::PlayerTurn {
            return;
        }
        self.typed_input.pop();
    }

    fn on_word_complete(&mut self) {
        self.words_typed += 1;
        
        if self.typed_input == self.current_word {
            self.words_correct += 1;
            self.combo += 1;
            if self.combo > self.max_combo {
                self.max_combo = self.combo;
            }
            
            // Calculate damage based on typing performance
            let wpm = self.calculate_wpm();
            let accuracy = self.calculate_accuracy();
            let damage = self.calculate_damage(wpm, accuracy);
            
            self.enemy.current_hp -= damage;
            
            self.battle_log.push(format!(
                "✓ {} ({:.0} WPM, {:.0}% acc) - {} damage! [{}x combo]",
                self.current_word, wpm, accuracy * 100.0, damage, self.combo
            ));
            
            if self.enemy.current_hp <= 0 {
                self.enemy.current_hp = 0;
                self.phase = CombatPhase::Victory;
                self.finalize_result(true, false, false);
            } else {
                self.phase = CombatPhase::EnemyTurn;
            }
        } else {
            self.combo = 0;
            self.battle_log.push(format!(
                "✗ Mistyped '{}' (typed '{}')",
                self.current_word, self.typed_input
            ));
            self.phase = CombatPhase::EnemyTurn;
        }
    }

    fn on_word_timeout(&mut self) {
        self.words_typed += 1;
        self.combo = 0;
        self.battle_log.push(format!(
            "⏰ Timeout! '{}' was too slow",
            self.current_word
        ));
        self.phase = CombatPhase::EnemyTurn;
    }

    pub fn execute_enemy_turn(&mut self, player: &mut Player) {
        if self.phase != CombatPhase::EnemyTurn {
            return;
        }

        let raw_damage = self.enemy.attack_power;
        let defense_reduction = (player.stats.vitality as f32 * 0.5).floor() as i32;
        let damage = (raw_damage - defense_reduction).max(1);
        let actual_damage = if self.player_shield > 0 {
            let absorbed = damage.min(self.player_shield);
            self.player_shield -= absorbed;
            damage - absorbed
        } else {
            damage
        };

        player.take_damage(actual_damage);
        
        // Get a random attack message
        let attack_msg = self.enemy.get_attack_message();
        self.battle_log.push(format!(
            "💥 {} {} for {} damage!",
            self.enemy.name, attack_msg, actual_damage
        ));

        if player.hp <= 0 {
            self.phase = CombatPhase::Defeat;
            self.finalize_result(false, false, false);
        } else {
            self.turn += 1;
            // Start next player turn with new content from game data
            self.current_word = if self.use_sentences {
                self.game_data.get_lore_sentence(self.floor, self.enemy.is_boss, Some(&self.enemy.name))
            } else {
                self.game_data.get_lore_word(self.floor, Some(&self.enemy.typing_theme))
            };
            
            // Adjust time based on content length
            self.time_limit = if self.use_sentences {
                15.0 + (self.current_word.len() as f32 * 0.1)
            } else {
                5.0 + (self.current_word.len() as f32 * 0.2)
            };
            
            self.typed_input.clear();
            self.time_remaining = self.time_limit;
            self.last_tick = Instant::now();
            self.typing_started = false;
            self.phase = CombatPhase::PlayerTurn;
        }
    }

    fn calculate_wpm(&self) -> f32 {
        if self.time_remaining >= self.time_limit {
            return 0.0;
        }
        let time_taken = self.time_limit - self.time_remaining;
        if time_taken <= 0.0 {
            return 0.0;
        }
        let words = self.current_word.len() as f32 / 5.0;
        let minutes = time_taken / 60.0;
        words / minutes
    }

    fn calculate_accuracy(&self) -> f32 {
        if self.total_chars == 0 {
            return 1.0;
        }
        self.correct_chars as f32 / self.total_chars as f32
    }

    fn calculate_damage(&self, wpm: f32, accuracy: f32) -> i32 {
        let base_damage = 10;
        
        // WPM bonus: +1 damage per 10 WPM above 30
        let wpm_bonus = ((wpm - 30.0) / 10.0).max(0.0) as i32;
        
        // Accuracy multiplier: 1.0 at 100%, 0.5 at 50%
        let accuracy_mult = 0.5 + (accuracy * 0.5);
        
        // Combo bonus: +10% per combo level (matches typing_feel system)
        // Max 3x damage at 20 combo
        let combo_mult = 1.0 + (self.combo as f32 * 0.1).min(2.0);
        
        let damage = (base_damage + wpm_bonus) as f32 * accuracy_mult * combo_mult;
        damage.round() as i32
    }

    pub fn try_flee(&mut self) -> bool {
        if self.enemy.is_boss {
            self.battle_log.push("Cannot flee from a boss!".to_string());
            return false;
        }
        
        let mut rng = rand::thread_rng();
        let flee_chance = 0.5; // 50% base flee chance
        
        if rng.gen::<f32>() < flee_chance {
            self.phase = CombatPhase::Fled;
            self.finalize_result(false, true, false);
            true
        } else {
            self.battle_log.push("Failed to flee!".to_string());
            self.phase = CombatPhase::EnemyTurn;
            false
        }
    }

    pub fn try_spare(&mut self) -> bool {
        // Undertale-style spare: can only spare when conditions are met
        if self.enemy.current_hp as f32 / self.enemy.max_hp as f32 > 0.25 {
            self.battle_log.push("The enemy isn't ready to be spared...".to_string());
            return false;
        }
        
        // Spare successful!
        self.phase = CombatPhase::Spared;
        self.finalize_result(true, false, true);
        true
    }

    fn finalize_result(&mut self, victory: bool, fled: bool, spared: bool) {
        let xp = if victory && !spared {
            self.enemy.xp_reward
        } else if spared {
            self.enemy.xp_reward / 2 // Half XP for sparing
        } else {
            0
        };
        
        let gold = if victory || spared {
            self.enemy.gold_reward
        } else {
            0
        };

        let accuracy = if self.words_typed > 0 {
            self.words_correct as f32 / self.words_typed as f32
        } else {
            0.0
        };

        self.result = Some(CombatResult {
            victory,
            fled,
            spared,
            xp_gained: xp,
            gold_gained: gold,
            turns_taken: self.turn,
            max_combo: self.max_combo,
            accuracy,
            avg_wpm: 0.0, // TODO: track average WPM
        });
    }

    pub fn get_accuracy(&self) -> f32 {
        if self.words_typed == 0 {
            return 100.0;
        }
        (self.words_correct as f32 / self.words_typed as f32) * 100.0
    }
}

/// Word pools for different difficulty levels
pub fn get_word_pool(difficulty: i32) -> Vec<String> {
    match difficulty {
        1 => vec![
            // Floor 1-2: Very easy words
            "the", "and", "for", "are", "but", "not", "you", "all",
            "can", "her", "was", "one", "our", "out", "day", "get",
            "has", "him", "his", "how", "its", "may", "new", "now",
            "old", "see", "two", "way", "who", "boy", "did", "own",
        ].into_iter().map(String::from).collect(),
        
        2 => vec![
            // Floor 3-4: Easy words
            "about", "after", "again", "being", "could", "every",
            "first", "found", "great", "house", "large", "learn",
            "never", "other", "place", "plant", "point", "right",
            "small", "sound", "spell", "still", "study", "their",
            "there", "these", "thing", "think", "three", "water",
            "where", "which", "world", "would", "write", "years",
        ].into_iter().map(String::from).collect(),
        
        3 => vec![
            // Floor 5-6: Medium words
            "because", "between", "country", "different", "example",
            "following", "government", "important", "information",
            "national", "political", "possible", "president", "problem",
            "question", "research", "service", "something", "together",
            "understand", "university", "everything", "experience",
        ].into_iter().map(String::from).collect(),
        
        4 => vec![
            // Floor 7-8: Hard words
            "administration", "approximately", "characteristic",
            "communication", "concentration", "consideration",
            "determination", "discrimination", "implementation",
            "infrastructure", "interpretation", "investigation",
            "pharmaceutical", "recommendation", "responsibility",
            "transformation", "understanding", "unfortunately",
        ].into_iter().map(String::from).collect(),
        
        _ => vec![
            // Floor 9+: Programming words!
            "algorithm", "asynchronous", "authentication", "blockchain",
            "compilation", "concurrency", "configuration", "cryptography",
            "database", "declaration", "dependency", "deployment",
            "encryption", "enumeration", "exception", "expression",
            "framework", "function", "garbage", "implementation",
            "inheritance", "initialization", "interface", "javascript",
            "kubernetes", "lambda", "middleware", "namespace",
            "optimization", "polymorphism", "protocol", "recursion",
            "refactoring", "repository", "serialization", "synchronous",
            "typescript", "validation", "virtualization", "webpack",
        ].into_iter().map(String::from).collect(),
    }
}

// Spell casting extensions for CombatState
impl CombatState {
    /// Toggle spell casting mode
    pub fn toggle_spell_mode(&mut self) {
        self.spell_mode = !self.spell_mode;
        if !self.spell_mode {
            self.selected_spell = None;
            self.spell_incantation = None;
        }
        self.typed_input.clear();
    }

    /// Select a spell by index and prepare to cast it
    pub fn select_spell(&mut self, spell: &super::spells::Spell) {
        self.spell_mode = true;
        self.spell_incantation = Some(spell.incantation.clone());
        self.current_word = spell.incantation.clone();
        self.typed_input.clear();
        self.time_remaining = spell.cast_time;
        self.time_limit = spell.cast_time;
        self.battle_log.push(format!("Casting {}... Type: {}", spell.name, spell.incantation));
    }

    /// Called when spell incantation is typed correctly
    pub fn cast_spell(&mut self, spell: &super::spells::Spell, player: &mut super::player::Player) -> bool {
        if player.mp < spell.mp_cost {
            self.battle_log.push("Not enough MP!".to_string());
            self.toggle_spell_mode();
            return false;
        }

        player.mp -= spell.mp_cost;
        
        match &spell.effect {
            super::spells::SpellEffect::Damage(dmg) => {
                let damage = (*dmg as f32 * (1.0 + player.stats.intellect as f32 * 0.05)) as i32;
                self.enemy.current_hp -= damage;
                self.battle_log.push(format!("✦ {} deals {} damage!", spell.name, damage));
            }
            super::spells::SpellEffect::Heal(heal) => {
                let amount = (*heal as f32 * (1.0 + player.stats.intellect as f32 * 0.03)) as i32;
                player.heal(amount);
                self.battle_log.push(format!("✦ {} restores {} HP!", spell.name, amount));
            }
            super::spells::SpellEffect::Shield(shield) => {
                self.player_shield += shield;
                self.battle_log.push(format!("✦ {} grants {} shield!", spell.name, shield));
            }
            super::spells::SpellEffect::Drain { damage, heal_percent } => {
                let dmg = (*damage as f32 * (1.0 + player.stats.intellect as f32 * 0.05)) as i32;
                self.enemy.current_hp -= dmg;
                let heal = dmg * heal_percent / 100;
                player.heal(heal);
                self.battle_log.push(format!("✦ {} drains {} life!", spell.name, dmg));
            }
            super::spells::SpellEffect::Multi { hits, damage_per_hit } => {
                let mut total = 0;
                for _ in 0..*hits {
                    let dmg = (*damage_per_hit as f32 * (1.0 + player.stats.intellect as f32 * 0.05)) as i32;
                    self.enemy.current_hp -= dmg;
                    total += dmg;
                }
                self.battle_log.push(format!("✦ {} hits {} times for {} total!", spell.name, hits, total));
            }
            _ => {
                self.battle_log.push(format!("✦ Cast {}!", spell.name));
            }
        }

        // Exit spell mode
        self.toggle_spell_mode();
        
        // Check for enemy defeat
        if self.enemy.current_hp <= 0 {
            self.phase = CombatPhase::Victory;
        }
        
        true
    }
}
