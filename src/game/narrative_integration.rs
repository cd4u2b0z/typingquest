//! Narrative Integration - Wires lore systems into actual gameplay
//!
//! This module connects all the narrative systems (deep_lore, lore_fragments,
//! encounter_writing, faction_system, voice_system) into the core game loop.
//! It's the glue that makes story appear during gameplay.

use std::collections::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::lore_fragments::{LoreJournal, build_lore_fragments};
use super::encounter_writing::{AuthoredEncounter, EncounterTracker, build_encounters};
use super::narrative::Chapter;

/// Central narrative coordinator - manages all story state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeEngine {
    /// Current chapter of the story
    pub chapter: Chapter,
    /// Player's progress through the mystery
    pub mystery_progress: MysteryProgress,
    /// Lore discovery tracking
    pub lore_journal: LoreJournal,
    /// Encounter tracking
    pub encounter_tracker: EncounterTracker,
    /// Faction standings (faction name -> reputation)
    pub faction_standings: HashMap<String, i32>,
    /// World state flags
    pub world_flags: HashMap<String, bool>,
    /// NPC opinion tracking
    pub npc_opinions: HashMap<String, i32>,
    /// Current location
    pub current_location: String,
    /// Time of day (0-23)
    pub time_of_day: u8,
    /// Weather
    pub weather: Weather,
    /// Story revelations player has seen
    pub revelations: Vec<String>,
    /// Pending narrative events
    pub pending_events: Vec<NarrativeEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    CorruptionMist,
}

/// Tracks player's progress through the identity mystery
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MysteryProgress {
    /// Current revelation tier (0-4)
    pub revelation_tier: u8,
    /// Clue IDs discovered
    pub clues_found: Vec<String>,
    /// Has player learned they are the First Speaker?
    pub identity_revealed: bool,
    /// Has player learned about their spouse?
    pub spouse_memory_unlocked: bool,
    /// Which ending path is player on?
    pub ending_tendency: EndingTendency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EndingTendency {
    #[default]
    Neutral,
    FinalSilence,    // Completing the Unwriting
    FirstWord,       // Reversing the Unwriting
    ThirdGrammar,    // Finding the synthesis
}

/// A narrative event that should trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeEvent {
    pub event_type: NarrativeEventType,
    pub priority: u8,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NarrativeEventType {
    LoreDiscovery { fragment_id: String },
    EncounterTrigger { encounter_id: String },
    FactionReaction { faction: String, change: i32 },
    DialogueUnlock { npc: String, dialogue_id: String },
    ChapterAdvance,
    RevelationMoment { revelation: String },
    MemoryFlash { memory_id: String },
    WorldStateChange { flag: String, value: bool },
}

impl Default for NarrativeEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl NarrativeEngine {
    pub fn new() -> Self {
        let mut faction_standings = HashMap::new();
        faction_standings.insert("scribes".to_string(), 0);
        faction_standings.insert("mechanists".to_string(), 0);
        faction_standings.insert("naturalists".to_string(), 0);
        faction_standings.insert("shadowwriters".to_string(), 0);
        faction_standings.insert("archivists".to_string(), 0);
        
        Self {
            chapter: Chapter::Awakening,
            mystery_progress: MysteryProgress::default(),
            lore_journal: LoreJournal::new(),
            encounter_tracker: EncounterTracker::new(),
            faction_standings,
            world_flags: HashMap::new(),
            npc_opinions: HashMap::new(),
            current_location: "haven".to_string(),
            time_of_day: 8,
            weather: Weather::Clear,
            revelations: Vec::new(),
            pending_events: Vec::new(),
        }
    }

    // ========================================================================
    // LOCATION & TIME MANAGEMENT
    // ========================================================================

    pub fn enter_location(&mut self, location: &str) {
        self.current_location = location.to_string();
        self.check_location_triggers(location);
    }

    pub fn advance_time(&mut self, hours: u8) {
        self.time_of_day = (self.time_of_day + hours) % 24;
        self.check_time_triggers();
    }

    pub fn set_weather(&mut self, weather: Weather) {
        self.weather = weather;
    }

    // ========================================================================
    // ENCOUNTER SYSTEM
    // ========================================================================

    /// Get available encounters for current location and state
    pub fn get_available_encounters(&self) -> Vec<String> {
        let all_encounters = build_encounters();
        let chapter_num = self.chapter_number();
        
        all_encounters.values()
            .filter(|e| self.encounter_available(e, chapter_num))
            .map(|e| e.id.clone())
            .collect()
    }

    /// Check if a specific encounter can trigger
    fn encounter_available(&self, encounter: &AuthoredEncounter, chapter: u32) -> bool {
        let reqs = &encounter.requirements;
        
        // Check if already completed (and not repeatable)
        if !encounter.repeatable && self.encounter_tracker.has_completed(&encounter.id) {
            return false;
        }
        
        // Check location
        if !encounter.valid_locations.contains(&self.current_location) {
            return false;
        }
        
        // Check chapter bounds
        if let Some(min) = reqs.min_chapter {
            if chapter < min {
                return false;
            }
        }
        if let Some(max) = reqs.max_chapter {
            if chapter > max {
                return false;
            }
        }
        
        // Check prerequisites
        if let Some(ref prereq) = reqs.prerequisite_encounter {
            if !self.encounter_tracker.has_completed(prereq) {
                return false;
            }
        }
        
        // Check blockers
        if let Some(ref blocker) = reqs.blocking_encounter {
            if self.encounter_tracker.has_completed(blocker) {
                return false;
            }
        }
        
        // Check required lore
        if let Some(ref lore) = reqs.required_lore {
            if !self.lore_journal.has_discovered(lore) {
                return false;
            }
        }
        
        // Check faction reputation
        if let Some((ref faction, min_rep)) = reqs.faction_reputation {
            let current_rep = self.get_faction_reputation(faction);
            if current_rep < min_rep {
                return false;
            }
        }
        
        true
    }

    /// Select an encounter based on current state (weighted random)
    pub fn select_encounter(&self) -> Option<String> {
        let available = self.get_available_encounters();
        if available.is_empty() {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        let all_encounters = build_encounters();
        
        // Weight by tags - major encounters less common
        let weights: Vec<f32> = available.iter().map(|id| {
            if let Some(e) = all_encounters.get(id) {
                if e.tags.contains(&"major".to_string()) {
                    0.3
                } else if e.tags.contains(&"player_mystery".to_string()) {
                    0.5
                } else {
                    1.0
                }
            } else {
                1.0
            }
        }).collect();
        
        let total: f32 = weights.iter().sum();
        let mut roll = rng.gen::<f32>() * total;
        
        for (i, weight) in weights.iter().enumerate() {
            roll -= weight;
            if roll <= 0.0 {
                return Some(available[i].clone());
            }
        }
        
        available.last().cloned()
    }

    pub fn complete_encounter(&mut self, encounter_id: &str, choice_id: &str) {
        self.encounter_tracker.complete_encounter(encounter_id, choice_id);
        
        // Apply consequences
        if let Some(encounter) = build_encounters().get(encounter_id) {
            let consequences = &encounter.consequences;
            
            // Apply reputation changes
            for (faction, change) in &consequences.reputation_changes {
                self.modify_faction_reputation(faction, *change);
            }
            
            // Reveal lore
            for lore_id in &consequences.lore_revealed {
                self.discover_lore(lore_id);
            }
            
            // NPC opinions
            for (npc, change) in &consequences.npc_opinion_changes {
                self.modify_npc_opinion(npc, *change);
            }
            
            // World state
            for flag in &consequences.world_state_changes {
                self.set_world_flag(flag, true);
            }
        }
    }

    // ========================================================================
    // LORE DISCOVERY
    // ========================================================================

    /// Discover a lore fragment
    pub fn discover_lore(&mut self, fragment_id: &str) {
        if self.lore_journal.has_discovered(fragment_id) {
            return;
        }
        
        self.lore_journal.discover(fragment_id);
        
        // Check if this advances the mystery
        self.check_mystery_progress(fragment_id);
        
        // Queue the discovery event
        self.pending_events.push(NarrativeEvent {
            event_type: NarrativeEventType::LoreDiscovery { 
                fragment_id: fragment_id.to_string() 
            },
            priority: 5,
            data: HashMap::new(),
        });
    }

    /// Get lore fragments available at current location
    pub fn get_discoverable_lore(&self) -> Vec<String> {
        let all_fragments = build_lore_fragments();
        
        all_fragments.values()
            .filter(|f| {
                !self.lore_journal.has_discovered(&f.id) &&
                f.location.to_lowercase().contains(&self.current_location.to_lowercase())
            })
            .map(|f| f.id.clone())
            .collect()
    }

    /// Check if discovering lore advances the mystery
    fn check_mystery_progress(&mut self, fragment_id: &str) {
        // Add to clues found
        if !self.mystery_progress.clues_found.contains(&fragment_id.to_string()) {
            self.mystery_progress.clues_found.push(fragment_id.to_string());
        }
        
        // Check for tier advancement based on clue count
        let clue_count = self.mystery_progress.clues_found.len();
        let new_tier = match clue_count {
            0..=2 => 0,
            3..=5 => 1,
            6..=9 => 2,
            10..=14 => 3,
            _ => 4,
        };
        
        if new_tier > self.mystery_progress.revelation_tier {
            self.mystery_progress.revelation_tier = new_tier;
            self.advance_mystery_tier();
        }
        
        // Special fragments trigger specific revelations
        match fragment_id {
            "player_previous_life" => {
                self.mystery_progress.identity_revealed = true;
                self.queue_revelation("You are the First Speaker, reborn.");
            }
            "first_speaker_journal_1" => {
                self.mystery_progress.spouse_memory_unlocked = true;
                self.queue_revelation("You loved someone once. You lost them.");
            }
            _ => {}
        }
    }

    fn advance_mystery_tier(&mut self) {
        let revelations = [
            "Something about your past doesn't add up...",
            "You've done this before. You're sure of it.",
            "The Corruption responds to you differently than others.",
            "You remember now. You remember everything.",
        ];
        
        let tier = self.mystery_progress.revelation_tier as usize;
        if tier > 0 && tier <= revelations.len() {
            self.queue_revelation(revelations[tier - 1]);
        }
    }

    fn queue_revelation(&mut self, revelation: &str) {
        self.revelations.push(revelation.to_string());
        self.pending_events.push(NarrativeEvent {
            event_type: NarrativeEventType::RevelationMoment {
                revelation: revelation.to_string(),
            },
            priority: 10,
            data: HashMap::new(),
        });
    }

    // ========================================================================
    // FACTION SYSTEM
    // ========================================================================

    pub fn get_faction_reputation(&self, faction: &str) -> i32 {
        *self.faction_standings.get(&faction.to_lowercase()).unwrap_or(&0)
    }

    pub fn modify_faction_reputation(&mut self, faction: &str, change: i32) {
        let key = faction.to_lowercase();
        let current = self.get_faction_reputation(&key);
        let new_value = (current + change).clamp(-100, 100);
        self.faction_standings.insert(key.clone(), new_value);
        
        // Check for faction-specific reactions
        self.check_faction_reactions(&key, change, new_value);
    }

    fn check_faction_reactions(&mut self, faction: &str, change: i32, new_rep: i32) {
        // Unlock faction content at thresholds
        if change > 0 {
            if new_rep >= 25 && new_rep - change < 25 {
                self.set_world_flag(&format!("{}_friendly", faction), true);
            }
            if new_rep >= 50 && new_rep - change < 50 {
                self.set_world_flag(&format!("{}_trusted", faction), true);
            }
            if new_rep >= 75 && new_rep - change < 75 {
                self.set_world_flag(&format!("{}_inner_circle", faction), true);
            }
        }
        
        // Negative thresholds
        if change < 0 {
            if new_rep <= -25 && new_rep - change > -25 {
                self.set_world_flag(&format!("{}_distrusted", faction), true);
            }
            if new_rep <= -50 && new_rep - change > -50 {
                self.set_world_flag(&format!("{}_hostile", faction), true);
            }
        }
    }

    // ========================================================================
    // NPC RELATIONSHIPS
    // ========================================================================

    pub fn get_npc_opinion(&self, npc: &str) -> i32 {
        *self.npc_opinions.get(npc).unwrap_or(&0)
    }

    pub fn modify_npc_opinion(&mut self, npc: &str, change: i32) {
        let current = self.get_npc_opinion(npc);
        self.npc_opinions.insert(npc.to_string(), (current + change).clamp(-100, 100));
    }

    pub fn meet_npc(&mut self, npc: &str) {
        self.encounter_tracker.meet_npc(npc);
        if !self.npc_opinions.contains_key(npc) {
            self.npc_opinions.insert(npc.to_string(), 0);
        }
    }

    // ========================================================================
    // WORLD STATE
    // ========================================================================

    pub fn set_world_flag(&mut self, flag: &str, value: bool) {
        self.world_flags.insert(flag.to_string(), value);
    }

    pub fn get_world_flag(&self, flag: &str) -> bool {
        *self.world_flags.get(flag).unwrap_or(&false)
    }

    // ========================================================================
    // CHAPTER MANAGEMENT
    // ========================================================================

    pub fn chapter_number(&self) -> u32 {
        match self.chapter {
            Chapter::Awakening => 1,
            Chapter::Discovery => 2,
            Chapter::Revelation => 3,
            Chapter::Allegiance => 4,
            Chapter::Conflict => 5,
            Chapter::Reckoning => 6,
        }
    }

    pub fn advance_chapter(&mut self) {
        self.chapter = match self.chapter {
            Chapter::Awakening => Chapter::Discovery,
            Chapter::Discovery => Chapter::Revelation,
            Chapter::Revelation => Chapter::Allegiance,
            Chapter::Allegiance => Chapter::Conflict,
            Chapter::Conflict => Chapter::Reckoning,
            Chapter::Reckoning => Chapter::Reckoning,
        };
        
        self.pending_events.push(NarrativeEvent {
            event_type: NarrativeEventType::ChapterAdvance,
            priority: 10,
            data: HashMap::new(),
        });
    }

    /// Check if chapter should advance based on progress
    pub fn check_chapter_advancement(&mut self) {
        let should_advance = match self.chapter {
            Chapter::Awakening => {
                self.encounter_tracker.completed_encounters.len() >= 3
            }
            Chapter::Discovery => {
                self.faction_standings.values().any(|&rep| rep >= 25)
            }
            Chapter::Revelation => {
                self.get_world_flag("deep_corruption_visited")
            }
            Chapter::Allegiance => {
                self.mystery_progress.identity_revealed
            }
            Chapter::Conflict => {
                self.get_world_flag("final_battle_ready")
            }
            Chapter::Reckoning => false,
        };
        
        if should_advance {
            self.advance_chapter();
        }
    }

    // ========================================================================
    // ENDING PATH
    // ========================================================================

    pub fn update_ending_tendency(&mut self) {
        let scribes = self.get_faction_reputation("scribes");
        let mechanists = self.get_faction_reputation("mechanists");
        let naturalists = self.get_faction_reputation("naturalists");
        
        if naturalists > scribes && naturalists > mechanists {
            if self.get_world_flag("accepted_cycle_of_death") {
                self.mystery_progress.ending_tendency = EndingTendency::FirstWord;
                return;
            }
        }
        
        if self.get_world_flag("embraced_corruption") {
            self.mystery_progress.ending_tendency = EndingTendency::FinalSilence;
            return;
        }
        
        let all_positive = scribes > 0 && mechanists > 0 && naturalists > 0;
        let high_mystery = self.mystery_progress.revelation_tier >= 3;
        
        if all_positive && high_mystery {
            self.mystery_progress.ending_tendency = EndingTendency::ThirdGrammar;
            return;
        }
        
        self.mystery_progress.ending_tendency = EndingTendency::Neutral;
    }

    // ========================================================================
    // EVENT PROCESSING
    // ========================================================================

    pub fn drain_events(&mut self) -> Vec<NarrativeEvent> {
        let mut events = std::mem::take(&mut self.pending_events);
        events.sort_by(|a, b| b.priority.cmp(&a.priority));
        events
    }

    fn check_location_triggers(&mut self, location: &str) {
        let visited_key = format!("visited_{}", location);
        if !self.get_world_flag(&visited_key) {
            self.set_world_flag(&visited_key, true);
            
            match location {
                "athenaeum" => {
                    self.pending_events.push(NarrativeEvent {
                        event_type: NarrativeEventType::MemoryFlash {
                            memory_id: "library_deja_vu".to_string(),
                        },
                        priority: 3,
                        data: HashMap::new(),
                    });
                }
                "corruption_zone" | "whispering_waste" => {
                    if !self.get_world_flag("deep_corruption_visited") {
                        self.set_world_flag("deep_corruption_visited", true);
                    }
                }
                _ => {}
            }
        }
    }

    fn check_time_triggers(&mut self) {
        if self.time_of_day == 0 && self.mystery_progress.revelation_tier >= 2 {
            self.pending_events.push(NarrativeEvent {
                event_type: NarrativeEventType::MemoryFlash {
                    memory_id: "midnight_dream".to_string(),
                },
                priority: 2,
                data: HashMap::new(),
            });
        }
    }
}

/// Generate contextual flavor text for combat
pub fn get_combat_flavor(
    engine: &NarrativeEngine,
    enemy_type: &str,
    context: CombatContext,
) -> String {
    let chapter = engine.chapter_number();
    let location = &engine.current_location;
    
    match context {
        CombatContext::Intro => {
            match (enemy_type, location.as_str()) {
                ("corrupted", "haven") => {
                    "It was human once. The corruption has rewritten its name.".to_string()
                }
                ("corrupted", _) => {
                    "Letters squirm beneath its skin. It speaks in broken syntax.".to_string()
                }
                ("boss", _) if chapter >= 4 => {
                    "You remember this one. You've fought before. Many times.".to_string()
                }
                _ => {
                    "Your fingers find the keys. Reality awaits your input.".to_string()
                }
            }
        }
        CombatContext::Victory => {
            if engine.mystery_progress.revelation_tier >= 3 {
                "The words flow through you. They always have. You just forgot.".to_string()
            } else {
                "Your typing holds true. The corruption recedes.".to_string()
            }
        }
        CombatContext::Defeat => {
            "The words scatter. Meaning fractures. You fall.".to_string()
        }
        CombatContext::HighCombo(combo) => {
            if combo >= 10 {
                "The words sing through your fingers. This is what you were made for.".to_string()
            } else {
                format!("{}x combo! The rhythm builds.", combo)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CombatContext {
    Intro,
    Victory,
    Defeat,
    HighCombo(i32),
}
