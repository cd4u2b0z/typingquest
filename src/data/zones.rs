//! Zone definitions and progression data
//!
//! Defines game zones, floor ranges, and unlock conditions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database of all zones
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZoneDatabase {
    pub zones: HashMap<String, Zone>,
}

/// A game zone with its configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub description: String,
    pub floor_start: u32,
    pub floor_end: u32,
    pub difficulty_modifier: f32,
    pub ambiance: ZoneAmbiance,
    pub enemy_pool: Vec<String>,
    pub boss_id: String,
    pub unlock_condition: UnlockCondition,
    pub rewards: ZoneRewards,
    pub special_mechanics: Vec<SpecialMechanic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneAmbiance {
    pub color_scheme: ColorScheme,
    pub music_track: String,
    pub ambient_description: String,
    pub flavor_texts: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary: (u8, u8, u8),
    pub secondary: (u8, u8, u8),
    pub accent: (u8, u8, u8),
    pub danger: (u8, u8, u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockCondition {
    Always,
    BossDefeated(String),
    FloorReached(u32),
    ItemCollected(String),
    AchievementUnlocked(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRewards {
    pub gold_multiplier: f32,
    pub xp_multiplier: f32,
    pub guaranteed_item: Option<String>,
    pub drop_pool: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecialMechanic {
    TimePressure { reduction_percent: f32 },
    DarkWords { hidden_chance: f32 },
    MirroredText { chance: f32 },
    SpeedVariation { min_mult: f32, max_mult: f32 },
    ElementalWeakness(String),
    HealingFountains { frequency: u32 },
    TrapWords { penalty_damage: i32 },
    ComboBonus { multiplier: f32 },
}

impl ZoneDatabase {
    pub fn embedded() -> Self {
        let mut zones = HashMap::new();

        // ═══════════════════════════════════════════════════════════════
        // ZONE 1: THE SILENT LIBRARY
        // ═══════════════════════════════════════════════════════════════
        zones.insert("silent_library".into(), Zone {
            id: "silent_library".into(),
            name: "The Silent Library".into(),
            description: "Once a grand repository of all written knowledge, now haunted by echoes of unspoken words.".into(),
            floor_start: 1,
            floor_end: 5,
            difficulty_modifier: 1.0,
            ambiance: ZoneAmbiance {
                color_scheme: ColorScheme {
                    primary: (139, 90, 43),
                    secondary: (218, 165, 32),
                    accent: (255, 248, 220),
                    danger: (139, 0, 0),
                },
                music_track: "library_whispers".into(),
                ambient_description: "Dust motes drift through shafts of pale light. Pages rustle with phantom fingers.".into(),
                flavor_texts: vec![
                    "The books seem to watch your passage.".into(),
                    "A faint scent of old parchment fills the air.".into(),
                    "Somewhere, a clock ticks steadily.".into(),
                    "The silence here is almost tangible.".into(),
                ],
            },
            enemy_pool: vec![
                "dust_sprite".into(),
                "paper_phantom".into(),
                "ink_wraith".into(),
                "bookworm".into(),
                "shelf_specter".into(),
            ],
            boss_id: "librarian_shade".into(),
            unlock_condition: UnlockCondition::Always,
            rewards: ZoneRewards {
                gold_multiplier: 1.0,
                xp_multiplier: 1.0,
                guaranteed_item: Some("quill_of_swiftness".into()),
                drop_pool: vec!["health_potion".into(), "mana_tonic".into()],
            },
            special_mechanics: vec![
                SpecialMechanic::HealingFountains { frequency: 3 },
            ],
        });

        // ═══════════════════════════════════════════════════════════════
        // ZONE 2: THE FORGOTTEN ARCHIVES
        // ═══════════════════════════════════════════════════════════════
        zones.insert("forgotten_archives".into(), Zone {
            id: "forgotten_archives".into(),
            name: "The Forgotten Archives".into(),
            description: "Deeper chambers where forbidden texts are sealed. The air grows cold with ancient secrets.".into(),
            floor_start: 6,
            floor_end: 10,
            difficulty_modifier: 1.25,
            ambiance: ZoneAmbiance {
                color_scheme: ColorScheme {
                    primary: (47, 79, 79),
                    secondary: (112, 128, 144),
                    accent: (176, 196, 222),
                    danger: (178, 34, 34),
                },
                music_track: "archive_depths".into(),
                ambient_description: "Crystalline formations cast prismatic shadows. Whispers echo from sealed vaults.".into(),
                flavor_texts: vec![
                    "These texts were sealed for good reason.".into(),
                    "The walls seem to shift when you're not looking.".into(),
                    "A chill runs down your spine.".into(),
                    "Knowledge has weight here - you can feel it.".into(),
                ],
            },
            enemy_pool: vec![
                "archive_guardian".into(),
                "sealed_secret".into(),
                "frost_cipher".into(),
                "crystal_scribe".into(),
                "vault_specter".into(),
            ],
            boss_id: "keeper_of_secrets".into(),
            unlock_condition: UnlockCondition::BossDefeated("librarian_shade".into()),
            rewards: ZoneRewards {
                gold_multiplier: 1.3,
                xp_multiplier: 1.25,
                guaranteed_item: Some("tome_of_power".into()),
                drop_pool: vec!["health_potion".into(), "scroll_of_clarity".into(), "lucky_coin".into()],
            },
            special_mechanics: vec![
                SpecialMechanic::TimePressure { reduction_percent: 10.0 },
                SpecialMechanic::DarkWords { hidden_chance: 0.1 },
            ],
        });

        // ═══════════════════════════════════════════════════════════════
        // ZONE 3: THE BURNING SCRIPTORIUM
        // ═══════════════════════════════════════════════════════════════
        zones.insert("burning_scriptorium".into(), Zone {
            id: "burning_scriptorium".into(),
            name: "The Burning Scriptorium".into(),
            description: "Where forbidden knowledge meets its end. Flames consume endlessly, yet nothing is destroyed.".into(),
            floor_start: 11,
            floor_end: 15,
            difficulty_modifier: 1.5,
            ambiance: ZoneAmbiance {
                color_scheme: ColorScheme {
                    primary: (139, 69, 19),
                    secondary: (255, 140, 0),
                    accent: (255, 215, 0),
                    danger: (220, 20, 60),
                },
                music_track: "eternal_flames".into(),
                ambient_description: "Fire dances on every surface, yet the heat is strangely bearable. Burning words float like embers.".into(),
                flavor_texts: vec![
                    "The flames speak in a language older than words.".into(),
                    "Ashes swirl into patterns that almost make sense.".into(),
                    "Everything burns, yet nothing is consumed.".into(),
                    "Heat shimmers make the text dance before your eyes.".into(),
                ],
            },
            enemy_pool: vec![
                "ember_sprite".into(),
                "flame_scrivener".into(),
                "ash_wraith".into(),
                "burning_tome".into(),
                "inferno_imp".into(),
            ],
            boss_id: "phoenix_chronicler".into(),
            unlock_condition: UnlockCondition::BossDefeated("keeper_of_secrets".into()),
            rewards: ZoneRewards {
                gold_multiplier: 1.5,
                xp_multiplier: 1.5,
                guaranteed_item: Some("inkblade".into()),
                drop_pool: vec!["greater_health_potion".into(), "mana_tonic".into(), "scholars_lens".into()],
            },
            special_mechanics: vec![
                SpecialMechanic::TimePressure { reduction_percent: 15.0 },
                SpecialMechanic::TrapWords { penalty_damage: 5 },
                SpecialMechanic::ElementalWeakness("ice".into()),
            ],
        });

        // ═══════════════════════════════════════════════════════════════
        // ZONE 4: THE CRYSTAL CODEX
        // ═══════════════════════════════════════════════════════════════
        zones.insert("crystal_codex".into(), Zone {
            id: "crystal_codex".into(),
            name: "The Crystal Codex".into(),
            description: "A realm where words are frozen in time, preserved in perfect crystalline structures.".into(),
            floor_start: 16,
            floor_end: 20,
            difficulty_modifier: 1.75,
            ambiance: ZoneAmbiance {
                color_scheme: ColorScheme {
                    primary: (135, 206, 235),
                    secondary: (224, 255, 255),
                    accent: (255, 255, 255),
                    danger: (65, 105, 225),
                },
                music_track: "frozen_words".into(),
                ambient_description: "Crystalline formations hold ancient text frozen mid-sentence. Each step echoes infinitely.".into(),
                flavor_texts: vec![
                    "Time moves strangely here.".into(),
                    "You can see your breath form words.".into(),
                    "The crystals hum with stored knowledge.".into(),
                    "Reflections show words you haven't typed yet.".into(),
                ],
            },
            enemy_pool: vec![
                "crystal_guardian".into(),
                "frozen_thought".into(),
                "time_shard".into(),
                "prism_wraith".into(),
                "echo_keeper".into(),
            ],
            boss_id: "chronoscribe".into(),
            unlock_condition: UnlockCondition::BossDefeated("phoenix_chronicler".into()),
            rewards: ZoneRewards {
                gold_multiplier: 1.75,
                xp_multiplier: 1.75,
                guaranteed_item: Some("void_vestments".into()),
                drop_pool: vec!["greater_health_potion".into(), "elixir_of_focus".into(), "heart_of_the_scribe".into()],
            },
            special_mechanics: vec![
                SpecialMechanic::MirroredText { chance: 0.15 },
                SpecialMechanic::SpeedVariation { min_mult: 0.8, max_mult: 1.3 },
                SpecialMechanic::ComboBonus { multiplier: 1.5 },
            ],
        });

        // ═══════════════════════════════════════════════════════════════
        // ZONE 5: THE VOID BETWEEN PAGES
        // ═══════════════════════════════════════════════════════════════
        zones.insert("void_between".into(), Zone {
            id: "void_between".into(),
            name: "The Void Between Pages".into(),
            description: "The space between written words, where meaning dissolves into pure potential.".into(),
            floor_start: 21,
            floor_end: 25,
            difficulty_modifier: 2.0,
            ambiance: ZoneAmbiance {
                color_scheme: ColorScheme {
                    primary: (25, 25, 112),
                    secondary: (75, 0, 130),
                    accent: (138, 43, 226),
                    danger: (255, 0, 255),
                },
                music_track: "void_whispers".into(),
                ambient_description: "Reality itself seems uncertain. Words form and dissolve in the darkness between thoughts.".into(),
                flavor_texts: vec![
                    "You're not sure if you're reading or being read.".into(),
                    "The void stares back, and it's literate.".into(),
                    "Words here have weight - and teeth.".into(),
                    "Your own thoughts echo back as typed text.".into(),
                ],
            },
            enemy_pool: vec![
                "void_crawler".into(),
                "null_word".into(),
                "entropy_wisp".into(),
                "blank_horror".into(),
                "unword".into(),
            ],
            boss_id: "void_herald".into(),
            unlock_condition: UnlockCondition::BossDefeated("chronoscribe".into()),
            rewards: ZoneRewards {
                gold_multiplier: 2.0,
                xp_multiplier: 2.0,
                guaranteed_item: Some("voidwriter".into()),
                drop_pool: vec!["supreme_health_potion".into(), "scroll_of_skip".into(), "void_fragment".into()],
            },
            special_mechanics: vec![
                SpecialMechanic::TimePressure { reduction_percent: 20.0 },
                SpecialMechanic::DarkWords { hidden_chance: 0.25 },
                SpecialMechanic::MirroredText { chance: 0.1 },
                SpecialMechanic::TrapWords { penalty_damage: 10 },
            ],
        });

        // ═══════════════════════════════════════════════════════════════
        // ZONE 6: THE GENESIS ARCHIVE
        // ═══════════════════════════════════════════════════════════════
        zones.insert("genesis_archive".into(), Zone {
            id: "genesis_archive".into(),
            name: "The Genesis Archive".into(),
            description: "Where the first words were written. Here, language itself was born.".into(),
            floor_start: 26,
            floor_end: 30,
            difficulty_modifier: 2.5,
            ambiance: ZoneAmbiance {
                color_scheme: ColorScheme {
                    primary: (255, 215, 0),
                    secondary: (255, 255, 255),
                    accent: (255, 250, 205),
                    danger: (0, 0, 0),
                },
                music_track: "first_words".into(),
                ambient_description: "Pure light streams from every surface. Words here are the fundamental particles of reality.".into(),
                flavor_texts: vec![
                    "In the beginning was the Word...".into(),
                    "Every letter you type reshapes existence.".into(),
                    "The boundary between thought and reality blurs.".into(),
                    "You are writing the world into being.".into(),
                ],
            },
            enemy_pool: vec![
                "prime_letter".into(),
                "genesis_construct".into(),
                "first_thought".into(),
                "alpha_word".into(),
                "creation_spark".into(),
            ],
            boss_id: "author_of_all".into(),
            unlock_condition: UnlockCondition::BossDefeated("void_herald".into()),
            rewards: ZoneRewards {
                gold_multiplier: 3.0,
                xp_multiplier: 3.0,
                guaranteed_item: Some("word_of_creation".into()),
                drop_pool: vec!["supreme_health_potion".into(), "combo_crown".into(), "genesis_shard".into()],
            },
            special_mechanics: vec![
                SpecialMechanic::TimePressure { reduction_percent: 25.0 },
                SpecialMechanic::ComboBonus { multiplier: 2.0 },
                SpecialMechanic::SpeedVariation { min_mult: 0.7, max_mult: 1.5 },
            ],
        });

        Self { zones }
    }

    pub fn get_zone(&self, id: &str) -> Option<&Zone> {
        self.zones.get(id)
    }

    pub fn get_zone_for_floor(&self, floor: u32) -> Option<&Zone> {
        self.zones.values().find(|z| floor >= z.floor_start && floor <= z.floor_end)
    }

    pub fn get_zones_in_order(&self) -> Vec<&Zone> {
        let mut zones: Vec<_> = self.zones.values().collect();
        zones.sort_by_key(|z| z.floor_start);
        zones
    }

    pub fn is_zone_unlocked(&self, zone_id: &str, defeated_bosses: &[String], floors_reached: u32, items: &[String], achievements: &[String]) -> bool {
        if let Some(zone) = self.zones.get(zone_id) {
            match &zone.unlock_condition {
                UnlockCondition::Always => true,
                UnlockCondition::BossDefeated(boss) => defeated_bosses.contains(boss),
                UnlockCondition::FloorReached(floor) => floors_reached >= *floor,
                UnlockCondition::ItemCollected(item) => items.contains(item),
                UnlockCondition::AchievementUnlocked(ach) => achievements.contains(ach),
            }
        } else {
            false
        }
    }
}

impl Zone {
    /// Get a random flavor text for this zone
    pub fn random_flavor_text(&self) -> &str {
        use rand::seq::SliceRandom;
        self.ambiance.flavor_texts.choose(&mut rand::thread_rng())
            .map(|s| s.as_str())
            .unwrap_or(&self.ambiance.ambient_description)
    }
    
    /// Get adjusted time limit for this zone
    pub fn adjust_time_limit(&self, base_time: f32) -> f32 {
        let mut adjusted = base_time;
        for mechanic in &self.special_mechanics {
            if let SpecialMechanic::TimePressure { reduction_percent } = mechanic {
                adjusted *= 1.0 - (reduction_percent / 100.0);
            }
        }
        adjusted
    }
    
    /// Check if words should be hidden
    pub fn should_hide_word(&self) -> bool {
        use rand::Rng;
        for mechanic in &self.special_mechanics {
            if let SpecialMechanic::DarkWords { hidden_chance } = mechanic {
                if rand::thread_rng().gen::<f32>() < *hidden_chance {
                    return true;
                }
            }
        }
        false
    }
    
    /// Check if text should be mirrored
    pub fn should_mirror_text(&self) -> bool {
        use rand::Rng;
        for mechanic in &self.special_mechanics {
            if let SpecialMechanic::MirroredText { chance } = mechanic {
                if rand::thread_rng().gen::<f32>() < *chance {
                    return true;
                }
            }
        }
        false
    }
}
