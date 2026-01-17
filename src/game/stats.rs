//! Statistics and Achievement Tracking System
//! 
//! Tracks all player statistics, achievements, and milestones.
//! Provides analytics for adaptive difficulty and player progression.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete statistics tracker for a player
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatsTracker {
    /// Session-specific stats (current run)
    pub session: SessionStats,
    
    /// Lifetime stats (across all runs)
    pub lifetime: LifetimeStats,
    
    /// Typing performance metrics
    pub typing: TypingStats,
    
    /// Combat performance metrics
    pub combat: CombatStats,
    
    /// Achievements and milestones
    pub achievements: AchievementTracker,
}

/// Statistics for the current game session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionStats {
    /// Session start time (unix timestamp)
    pub started_at: u64,
    
    /// Total time played this session (seconds)
    pub playtime_seconds: u64,
    
    /// Floors cleared this run
    pub floors_cleared: i32,
    
    /// Rooms explored this run
    pub rooms_explored: i32,
    
    /// Enemies defeated this run
    pub enemies_defeated: i32,
    
    /// Bosses defeated this run
    pub bosses_defeated: i32,
    
    /// Gold earned this run
    pub gold_earned: u64,
    
    /// Gold spent this run
    pub gold_spent: u64,
    
    /// Items collected this run
    pub items_collected: i32,
    
    /// Items used this run
    pub items_used: i32,
    
    /// Deaths this run (for non-permadeath modes)
    pub deaths: i32,
}

/// Lifetime statistics across all runs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LifetimeStats {
    /// Total runs started
    pub runs_started: i32,
    
    /// Total runs completed (victory)
    pub runs_completed: i32,
    
    /// Total playtime (seconds)
    pub total_playtime_seconds: u64,
    
    /// Highest floor ever reached
    pub highest_floor: i32,
    
    /// Total enemies defeated (all time)
    pub total_enemies_defeated: i32,
    
    /// Total bosses defeated (all time)
    pub total_bosses_defeated: i32,
    
    /// Total gold earned (all time)
    pub total_gold_earned: u64,
    
    /// Enemies defeated by type
    pub enemies_by_type: HashMap<String, i32>,
    
    /// Runs by class
    pub runs_by_class: HashMap<String, i32>,
    
    /// Wins by class
    pub wins_by_class: HashMap<String, i32>,
}

/// Typing-specific statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypingStats {
    /// Total words typed
    pub total_words: i32,
    
    /// Perfect words (no backspaces)
    pub perfect_words: i32,
    
    /// Total characters typed
    pub total_characters: i64,
    
    /// Correct characters typed
    pub correct_characters: i64,
    
    /// Backspaces used
    pub backspaces_used: i64,
    
    /// Words failed (time ran out)
    pub words_failed: i32,
    
    /// Best WPM achieved
    pub best_wpm: f32,
    
    /// Average WPM (rolling)
    pub average_wpm: f32,
    
    /// WPM sample count for average
    pub wpm_samples: i32,
    
    /// Best accuracy achieved (session)
    pub best_accuracy: f32,
    
    /// Average accuracy (rolling)
    pub average_accuracy: f32,
    
    /// Accuracy sample count
    pub accuracy_samples: i32,
    
    /// Longest streak
    pub longest_streak: i32,
    
    /// Current streak
    pub current_streak: i32,
    
    /// Highest combo achieved
    pub highest_combo: i32,
    
    /// Total combo damage bonus earned
    pub total_combo_bonus: i64,
}

impl TypingStats {
    /// Update WPM rolling average
    pub fn record_wpm(&mut self, wpm: f32) {
        if wpm > self.best_wpm {
            self.best_wpm = wpm;
        }
        
        // Rolling average
        self.wpm_samples += 1;
        let weight = 1.0 / self.wpm_samples as f32;
        self.average_wpm = self.average_wpm * (1.0 - weight) + wpm * weight;
    }
    
    /// Update accuracy rolling average
    pub fn record_accuracy(&mut self, accuracy: f32) {
        if accuracy > self.best_accuracy {
            self.best_accuracy = accuracy;
        }
        
        self.accuracy_samples += 1;
        let weight = 1.0 / self.accuracy_samples as f32;
        self.average_accuracy = self.average_accuracy * (1.0 - weight) + accuracy * weight;
    }
    
    /// Record a completed word
    pub fn record_word(&mut self, chars: i32, correct: i32, time_seconds: f32, was_perfect: bool) {
        self.total_words += 1;
        self.total_characters += chars as i64;
        self.correct_characters += correct as i64;
        
        if was_perfect {
            self.perfect_words += 1;
        }
        
        // Calculate WPM for this word (chars / 5 = "standard words")
        if time_seconds > 0.0 {
            let wpm = (chars as f32 / 5.0) / (time_seconds / 60.0);
            self.record_wpm(wpm);
        }
        
        // Update accuracy
        let accuracy = if chars > 0 {
            correct as f32 / chars as f32
        } else {
            1.0
        };
        self.record_accuracy(accuracy);
    }
    
    /// Update streak
    pub fn record_streak(&mut self, streak: i32) {
        self.current_streak = streak;
        if streak > self.longest_streak {
            self.longest_streak = streak;
        }
    }
    
    /// Update combo
    pub fn record_combo(&mut self, combo: i32) {
        if combo > self.highest_combo {
            self.highest_combo = combo;
        }
    }
    
    /// Overall accuracy percentage
    pub fn overall_accuracy(&self) -> f32 {
        if self.total_characters > 0 {
            self.correct_characters as f32 / self.total_characters as f32 * 100.0
        } else {
            100.0
        }
    }
    
    /// Perfect word percentage
    pub fn perfect_rate(&self) -> f32 {
        if self.total_words > 0 {
            self.perfect_words as f32 / self.total_words as f32 * 100.0
        } else {
            100.0
        }
    }
}

/// Combat-specific statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CombatStats {
    /// Total damage dealt
    pub total_damage_dealt: i64,
    
    /// Total damage taken
    pub total_damage_taken: i64,
    
    /// Critical hits landed
    pub critical_hits: i32,
    
    /// Flee attempts
    pub flee_attempts: i32,
    
    /// Successful flees
    pub successful_flees: i32,
    
    /// Spells cast
    pub spells_cast: i32,
    
    /// Items used in combat
    pub items_used: i32,
    
    /// One-shot kills (killed in single word)
    pub one_shot_kills: i32,
    
    /// Close calls (won with <10% HP)
    pub close_calls: i32,
    
    /// Flawless victories (no damage taken)
    pub flawless_victories: i32,
    
    /// Times knocked below 10% HP
    pub near_deaths: i32,
    
    /// Fastest boss kill (seconds)
    pub fastest_boss_kill: Option<f32>,
    
    /// Total combat time (seconds)
    pub total_combat_time: f32,
}

impl CombatStats {
    /// Record damage dealt
    pub fn record_damage_dealt(&mut self, amount: i32, was_critical: bool) {
        self.total_damage_dealt += amount as i64;
        if was_critical {
            self.critical_hits += 1;
        }
    }
    
    /// Record damage taken
    pub fn record_damage_taken(&mut self, amount: i32, hp_remaining: i32, max_hp: i32) {
        self.total_damage_taken += amount as i64;
        
        let hp_percent = hp_remaining as f32 / max_hp as f32;
        if hp_percent < 0.1 && hp_remaining > 0 {
            self.near_deaths += 1;
        }
    }
    
    /// Record combat victory
    pub fn record_victory(&mut self, damage_taken: i32, hp_remaining: i32, max_hp: i32, duration: f32, was_boss: bool) {
        self.total_combat_time += duration;
        
        if damage_taken == 0 {
            self.flawless_victories += 1;
        }
        
        let hp_percent = hp_remaining as f32 / max_hp as f32;
        if hp_percent < 0.1 {
            self.close_calls += 1;
        }
        
        if was_boss {
            match self.fastest_boss_kill {
                None => self.fastest_boss_kill = Some(duration),
                Some(fastest) if duration < fastest => self.fastest_boss_kill = Some(duration),
                _ => {}
            }
        }
    }
}

/// Achievement tracking
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AchievementTracker {
    /// Unlocked achievements
    pub unlocked: Vec<Achievement>,
    
    /// Progress on incomplete achievements
    pub progress: HashMap<String, AchievementProgress>,
}

/// An unlocked achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked_at: u64, // unix timestamp
    pub rarity: AchievementRarity,
}

/// Achievement rarity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

/// Progress toward an achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementProgress {
    pub current: i32,
    pub target: i32,
}

impl AchievementProgress {
    pub fn percent(&self) -> f32 {
        if self.target > 0 {
            (self.current as f32 / self.target as f32 * 100.0).min(100.0)
        } else {
            0.0
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.current >= self.target
    }
}

/// All defined achievements in the game
pub fn get_all_achievements() -> Vec<AchievementDefinition> {
    vec![
        // Typing achievements
        AchievementDefinition {
            id: "speed_demon".to_string(),
            name: "Speed Demon".to_string(),
            description: "Achieve 100+ WPM in combat".to_string(),
            rarity: AchievementRarity::Rare,
            condition: AchievementCondition::WpmThreshold(100.0),
        },
        AchievementDefinition {
            id: "perfectionist".to_string(),
            name: "Perfectionist".to_string(),
            description: "Type 50 words perfectly (no backspaces)".to_string(),
            rarity: AchievementRarity::Uncommon,
            condition: AchievementCondition::PerfectWords(50),
        },
        AchievementDefinition {
            id: "combo_master".to_string(),
            name: "Combo Master".to_string(),
            description: "Achieve a 20x combo".to_string(),
            rarity: AchievementRarity::Rare,
            condition: AchievementCondition::ComboThreshold(20),
        },
        AchievementDefinition {
            id: "on_fire".to_string(),
            name: "On Fire".to_string(),
            description: "Complete a 10 word streak".to_string(),
            rarity: AchievementRarity::Common,
            condition: AchievementCondition::StreakThreshold(10),
        },
        
        // Combat achievements
        AchievementDefinition {
            id: "slayer".to_string(),
            name: "Slayer".to_string(),
            description: "Defeat 100 enemies".to_string(),
            rarity: AchievementRarity::Common,
            condition: AchievementCondition::EnemiesDefeated(100),
        },
        AchievementDefinition {
            id: "boss_hunter".to_string(),
            name: "Boss Hunter".to_string(),
            description: "Defeat 10 bosses".to_string(),
            rarity: AchievementRarity::Uncommon,
            condition: AchievementCondition::BossesDefeated(10),
        },
        AchievementDefinition {
            id: "flawless".to_string(),
            name: "Flawless".to_string(),
            description: "Win a combat without taking damage".to_string(),
            rarity: AchievementRarity::Uncommon,
            condition: AchievementCondition::FlawlessVictories(1),
        },
        AchievementDefinition {
            id: "close_call".to_string(),
            name: "Close Call".to_string(),
            description: "Win with less than 10% HP".to_string(),
            rarity: AchievementRarity::Common,
            condition: AchievementCondition::CloseCallVictories(1),
        },
        
        // Progression achievements
        AchievementDefinition {
            id: "dungeon_diver".to_string(),
            name: "Dungeon Diver".to_string(),
            description: "Reach floor 5".to_string(),
            rarity: AchievementRarity::Common,
            condition: AchievementCondition::FloorReached(5),
        },
        AchievementDefinition {
            id: "deep_delver".to_string(),
            name: "Deep Delver".to_string(),
            description: "Reach floor 10".to_string(),
            rarity: AchievementRarity::Uncommon,
            condition: AchievementCondition::FloorReached(10),
        },
        AchievementDefinition {
            id: "champion".to_string(),
            name: "Champion".to_string(),
            description: "Complete a run".to_string(),
            rarity: AchievementRarity::Rare,
            condition: AchievementCondition::RunsCompleted(1),
        },
        AchievementDefinition {
            id: "veteran".to_string(),
            name: "Veteran".to_string(),
            description: "Complete 5 runs".to_string(),
            rarity: AchievementRarity::Epic,
            condition: AchievementCondition::RunsCompleted(5),
        },
        
        // Special achievements
        AchievementDefinition {
            id: "wordsmith_master".to_string(),
            name: "Wordsmith Master".to_string(),
            description: "Complete a run as Wordsmith".to_string(),
            rarity: AchievementRarity::Uncommon,
            condition: AchievementCondition::ClassVictory("Wordsmith".to_string()),
        },
        AchievementDefinition {
            id: "millionaire".to_string(),
            name: "Millionaire".to_string(),
            description: "Accumulate 10,000 gold total".to_string(),
            rarity: AchievementRarity::Rare,
            condition: AchievementCondition::TotalGold(10000),
        },
    ]
}

/// Definition of an achievement (not yet unlocked)
#[derive(Debug, Clone)]
pub struct AchievementDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: AchievementRarity,
    pub condition: AchievementCondition,
}

/// Conditions for unlocking achievements
#[derive(Debug, Clone)]
pub enum AchievementCondition {
    WpmThreshold(f32),
    PerfectWords(i32),
    ComboThreshold(i32),
    StreakThreshold(i32),
    EnemiesDefeated(i32),
    BossesDefeated(i32),
    FlawlessVictories(i32),
    CloseCallVictories(i32),
    FloorReached(i32),
    RunsCompleted(i32),
    ClassVictory(String),
    TotalGold(u64),
}

impl AchievementTracker {
    /// Check if an achievement is unlocked
    pub fn has_achievement(&self, id: &str) -> bool {
        self.unlocked.iter().any(|a| a.id == id)
    }
    
    /// Check all achievements against current stats
    pub fn check_achievements(&mut self, stats: &StatsTracker) -> Vec<Achievement> {
        let mut newly_unlocked = Vec::new();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        for def in get_all_achievements() {
            if self.has_achievement(&def.id) {
                continue;
            }
            
            let unlocked = match &def.condition {
                AchievementCondition::WpmThreshold(wpm) => stats.typing.best_wpm >= *wpm,
                AchievementCondition::PerfectWords(count) => stats.typing.perfect_words >= *count,
                AchievementCondition::ComboThreshold(combo) => stats.typing.highest_combo >= *combo,
                AchievementCondition::StreakThreshold(streak) => stats.typing.longest_streak >= *streak,
                AchievementCondition::EnemiesDefeated(count) => stats.lifetime.total_enemies_defeated >= *count,
                AchievementCondition::BossesDefeated(count) => stats.lifetime.total_bosses_defeated >= *count,
                AchievementCondition::FlawlessVictories(count) => stats.combat.flawless_victories >= *count,
                AchievementCondition::CloseCallVictories(count) => stats.combat.close_calls >= *count,
                AchievementCondition::FloorReached(floor) => stats.lifetime.highest_floor >= *floor,
                AchievementCondition::RunsCompleted(count) => stats.lifetime.runs_completed >= *count,
                AchievementCondition::ClassVictory(class) => {
                    stats.lifetime.wins_by_class.get(class).copied().unwrap_or(0) > 0
                }
                AchievementCondition::TotalGold(amount) => stats.lifetime.total_gold_earned >= *amount,
            };
            
            if unlocked {
                let achievement = Achievement {
                    id: def.id.clone(),
                    name: def.name.clone(),
                    description: def.description.clone(),
                    unlocked_at: now,
                    rarity: def.rarity,
                };
                self.unlocked.push(achievement.clone());
                newly_unlocked.push(achievement);
            }
        }
        
        newly_unlocked
    }
}
