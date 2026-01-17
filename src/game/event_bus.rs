//! Event Bus System - Central nervous system for game events
//!
//! All game systems communicate through events rather than direct coupling.
//! This allows emergent behavior: a combat event triggers faction reactions,
//! which trigger NPC dialogue changes, which affect quest availability.
//!
//! Inspired by: Entity-Component-System architecture, pub/sub messaging

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use super::narrative::Faction;

/// Unique identifier for event subscriptions
pub type SubscriptionId = u64;

/// Central event bus for game-wide communication
#[derive(Debug, Clone)]
pub struct EventBus {
    /// Queue of pending events
    queue: VecDeque<GameEvent>,
    /// Event history for this run
    history: Vec<GameEventRecord>,
    /// Maximum history size
    max_history: usize,
    /// Current event ID counter
    event_counter: u64,
    /// Deferred events (trigger after delay)
    deferred: Vec<DeferredEvent>,
    /// Event statistics
    stats: EventStats,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            history: Vec::new(),
            max_history: 1000,
            event_counter: 0,
            deferred: Vec::new(),
            stats: EventStats::default(),
        }
    }
    
    /// Emit an event immediately
    pub fn emit(&mut self, event: GameEvent) {
        self.event_counter += 1;
        let record = GameEventRecord {
            id: self.event_counter,
            event: event.clone(),
            timestamp: std::time::Instant::now(),
        };
        
        self.queue.push_back(event.clone());
        self.history.push(record);
        self.stats.record_event(&event);
        
        // Trim history if needed
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }
    
    /// Emit an event after a delay (in game ticks)
    pub fn emit_deferred(&mut self, event: GameEvent, delay_ticks: u32) {
        self.deferred.push(DeferredEvent {
            event,
            ticks_remaining: delay_ticks,
        });
    }
    
    /// Process one tick for deferred events
    pub fn tick(&mut self) {
        let mut ready: Vec<GameEvent> = Vec::new();
        
        self.deferred.retain_mut(|d| {
            d.ticks_remaining = d.ticks_remaining.saturating_sub(1);
            if d.ticks_remaining == 0 {
                ready.push(d.event.clone());
                false
            } else {
                true
            }
        });
        
        for event in ready {
            self.emit(event);
        }
    }
    
    /// Get next pending event
    pub fn poll(&mut self) -> Option<GameEvent> {
        self.queue.pop_front()
    }
    
    /// Check if there are pending events
    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty()
    }
    
    /// Get all pending events (for batch processing)
    pub fn drain_all(&mut self) -> Vec<GameEvent> {
        self.queue.drain(..).collect()
    }
    
    /// Query history for specific event types
    pub fn query_history(&self, filter: EventFilter) -> Vec<&GameEventRecord> {
        self.history.iter().filter(|r| filter.matches(&r.event)).collect()
    }
    
    /// Get event statistics
    pub fn stats(&self) -> &EventStats {
        &self.stats
    }
    
    /// Clear all pending events
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

/// Record of a game event with metadata
#[derive(Debug, Clone)]
pub struct GameEventRecord {
    pub id: u64,
    pub event: GameEvent,
    pub timestamp: std::time::Instant,
}

/// Deferred event waiting to trigger
#[derive(Debug, Clone)]
struct DeferredEvent {
    event: GameEvent,
    ticks_remaining: u32,
}

/// All possible game events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    // === Player Actions ===
    PlayerMoved { from: String, to: String },
    PlayerTyped { text: String, wpm: f32, accuracy: f32, context: String },
    PlayerLeveledUp { new_level: u32, skill_points: u32 },
    PlayerDied { cause: DeathCause, location: String },
    PlayerRested { hours: u32, location: String },
    
    // === Combat Events ===
    CombatStarted { enemy: String, location: String },
    CombatEnded { enemy: String, outcome: CombatOutcome },
    DamageDealt { source: String, target: String, amount: i32, damage_type: String },
    DamageTaken { source: String, amount: i32, damage_type: String },
    SpellCast { caster: String, spell: String, target: Option<String> },
    ComboAchieved { count: u32, bonus: f32 },
    PerfectWord { word: String, bonus_xp: u32 },
    
    // === Faction Events ===
    FactionStandingChanged { faction: Faction, old_standing: i32, new_standing: i32, reason: String },
    FactionJoined { faction: Faction, rank: String },
    FactionPromoted { faction: Faction, old_rank: String, new_rank: String },
    FactionBetrayedBy { faction: Faction, consequence: String },
    FactionBountyPlaced { faction: Faction, amount: i32 },
    FactionWarDeclared { aggressor: Faction, defender: Faction },
    
    // === NPC Events ===
    NPCMet { npc: String, location: String },
    NPCDialogue { npc: String, topic: String },
    NPCRelationshipChanged { npc: String, old_value: i32, new_value: i32 },
    NPCSecretRevealed { npc: String, secret: String },
    NPCDied { npc: String, cause: String },
    NPCBetrayedPlayer { npc: String, reason: String },
    
    // === Quest Events ===
    QuestStarted { quest_id: String, quest_name: String },
    QuestStageCompleted { quest_id: String, stage: u32 },
    QuestCompleted { quest_id: String, outcome: String },
    QuestFailed { quest_id: String, reason: String },
    QuestObjectiveDiscovered { quest_id: String, objective: String },
    
    // === World Events ===
    LocationDiscovered { location: String, region: String },
    LocationEntered { location: String },
    WorldCorruptionChanged { old_level: f32, new_level: f32, location: Option<String> },
    TimePassedHours { hours: u32 },
    TimePassedDays { days: u32 },
    WeatherChanged { new_weather: String, location: String },
    RandomEncounter { encounter_type: String, location: String },
    
    // === Item Events ===
    ItemAcquired { item: String, quantity: u32, source: String },
    ItemLost { item: String, quantity: u32, reason: String },
    ItemUsed { item: String, effect: String },
    ItemCrafted { item: String, recipe: String },
    GoldChanged { old_amount: i32, new_amount: i32, reason: String },
    
    // === Skill Events ===
    SkillUnlocked { skill: String, tree: String },
    SkillUsed { skill: String, effect: String },
    ExperienceGained { amount: u32, source: String },
    
    // === Narrative Events ===
    ChapterStarted { chapter: u32, title: String },
    ChapterEnded { chapter: u32, outcome: String },
    ProphecyProgressed { prophecy_id: String, progress: f32 },
    ProphecyFulfilled { prophecy_id: String, outcome: String },
    MajorChoiceMade { choice_id: String, option: String },
    LoreDiscovered { lore_id: String, category: String },
    
    // === Meta Events ===
    GameSaved { slot: String },
    GameLoaded { slot: String },
    AchievementUnlocked { achievement: String },
    StatisticUpdated { stat: String, new_value: u64 },
    
    // === Dr. Baklava Events (Easter Eggs) ===
    DrBaklavaEvent { event_type: DrBaklavaEventType },
}

impl GameEvent {
    pub fn category(&self) -> EventCategory {
        match self {
            Self::PlayerMoved { .. } |
            Self::PlayerTyped { .. } |
            Self::PlayerLeveledUp { .. } |
            Self::PlayerDied { .. } |
            Self::PlayerRested { .. } => EventCategory::Player,
            
            Self::CombatStarted { .. } |
            Self::CombatEnded { .. } |
            Self::DamageDealt { .. } |
            Self::DamageTaken { .. } |
            Self::SpellCast { .. } |
            Self::ComboAchieved { .. } |
            Self::PerfectWord { .. } => EventCategory::Combat,
            
            Self::FactionStandingChanged { .. } |
            Self::FactionJoined { .. } |
            Self::FactionPromoted { .. } |
            Self::FactionBetrayedBy { .. } |
            Self::FactionBountyPlaced { .. } |
            Self::FactionWarDeclared { .. } => EventCategory::Faction,
            
            Self::NPCMet { .. } |
            Self::NPCDialogue { .. } |
            Self::NPCRelationshipChanged { .. } |
            Self::NPCSecretRevealed { .. } |
            Self::NPCDied { .. } |
            Self::NPCBetrayedPlayer { .. } => EventCategory::NPC,
            
            Self::QuestStarted { .. } |
            Self::QuestStageCompleted { .. } |
            Self::QuestCompleted { .. } |
            Self::QuestFailed { .. } |
            Self::QuestObjectiveDiscovered { .. } => EventCategory::Quest,
            
            Self::LocationDiscovered { .. } |
            Self::LocationEntered { .. } |
            Self::WorldCorruptionChanged { .. } |
            Self::TimePassedHours { .. } |
            Self::TimePassedDays { .. } |
            Self::WeatherChanged { .. } |
            Self::RandomEncounter { .. } => EventCategory::World,
            
            Self::ItemAcquired { .. } |
            Self::ItemLost { .. } |
            Self::ItemUsed { .. } |
            Self::ItemCrafted { .. } |
            Self::GoldChanged { .. } => EventCategory::Item,
            
            Self::SkillUnlocked { .. } |
            Self::SkillUsed { .. } |
            Self::ExperienceGained { .. } => EventCategory::Skill,
            
            Self::ChapterStarted { .. } |
            Self::ChapterEnded { .. } |
            Self::ProphecyProgressed { .. } |
            Self::ProphecyFulfilled { .. } |
            Self::MajorChoiceMade { .. } |
            Self::LoreDiscovered { .. } => EventCategory::Narrative,
            
            Self::GameSaved { .. } |
            Self::GameLoaded { .. } |
            Self::AchievementUnlocked { .. } |
            Self::StatisticUpdated { .. } => EventCategory::Meta,
            
            Self::DrBaklavaEvent { .. } => EventCategory::Easter,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventCategory {
    Player,
    Combat,
    Faction,
    NPC,
    Quest,
    World,
    Item,
    Skill,
    Narrative,
    Meta,
    Easter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeathCause {
    Combat { enemy: String },
    Corruption,
    Trap { trap_type: String },
    Ritual { ritual_name: String },
    Starvation,
    Fall,
    DrBaklava, // Easter egg death
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatOutcome {
    Victory { xp_gained: u32, loot: Vec<String> },
    Defeat,
    Fled,
    Negotiated { terms: String },
    Interrupted { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrBaklavaEventType {
    /// The mysterious figure appears
    Sighting { location: String },
    /// Found one of his recipes
    RecipeFound { recipe_name: String },
    /// His name mentioned in lore
    LoreMention { context: String },
    /// A pastry-based attack
    PastryAttack { damage: i32 },
    /// Secret achievement unlocked
    SecretAchievement { achievement: String },
}

/// Filter for querying event history
#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    pub categories: Option<Vec<EventCategory>>,
    pub factions: Option<Vec<Faction>>,
    pub npcs: Option<Vec<String>>,
    pub locations: Option<Vec<String>>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn category(mut self, cat: EventCategory) -> Self {
        self.categories.get_or_insert_with(Vec::new).push(cat);
        self
    }
    
    pub fn faction(mut self, faction: Faction) -> Self {
        self.factions.get_or_insert_with(Vec::new).push(faction);
        self
    }
    
    pub fn npc(mut self, npc: String) -> Self {
        self.npcs.get_or_insert_with(Vec::new).push(npc);
        self
    }
    
    pub fn matches(&self, event: &GameEvent) -> bool {
        // Check category filter
        if let Some(cats) = &self.categories {
            if !cats.contains(&event.category()) {
                return false;
            }
        }
        
        // Check faction filter
        if let Some(factions) = &self.factions {
            match event {
                GameEvent::FactionStandingChanged { faction, .. } |
                GameEvent::FactionJoined { faction, .. } |
                GameEvent::FactionPromoted { faction, .. } |
                GameEvent::FactionBetrayedBy { faction, .. } |
                GameEvent::FactionBountyPlaced { faction, .. } => {
                    if !factions.contains(faction) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        
        // Check NPC filter
        if let Some(npcs) = &self.npcs {
            match event {
                GameEvent::NPCMet { npc, .. } |
                GameEvent::NPCDialogue { npc, .. } |
                GameEvent::NPCRelationshipChanged { npc, .. } |
                GameEvent::NPCSecretRevealed { npc, .. } |
                GameEvent::NPCDied { npc, .. } |
                GameEvent::NPCBetrayedPlayer { npc, .. } => {
                    if !npcs.contains(npc) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        
        true
    }
}

/// Statistics about events for this run
#[derive(Debug, Clone, Default)]
pub struct EventStats {
    pub total_events: u64,
    pub events_by_category: HashMap<EventCategory, u64>,
    pub damage_dealt: i64,
    pub damage_taken: i64,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub npcs_met: u32,
    pub locations_discovered: u32,
    pub quests_completed: u32,
    pub quests_failed: u32,
    pub deaths: u32,
    pub words_typed: u64,
    pub perfect_words: u64,
    pub combats_won: u32,
    pub combats_lost: u32,
    pub faction_changes: u32,
}

impl EventStats {
    pub fn record_event(&mut self, event: &GameEvent) {
        self.total_events += 1;
        *self.events_by_category.entry(event.category()).or_insert(0) += 1;
        
        match event {
            GameEvent::DamageDealt { amount, .. } => {
                self.damage_dealt += *amount as i64;
            }
            GameEvent::DamageTaken { amount, .. } => {
                self.damage_taken += *amount as i64;
            }
            GameEvent::GoldChanged { old_amount, new_amount, .. } => {
                let diff = new_amount - old_amount;
                if diff > 0 {
                    self.gold_earned += diff as i64;
                } else {
                    self.gold_spent += (-diff) as i64;
                }
            }
            GameEvent::NPCMet { .. } => {
                self.npcs_met += 1;
            }
            GameEvent::LocationDiscovered { .. } => {
                self.locations_discovered += 1;
            }
            GameEvent::QuestCompleted { .. } => {
                self.quests_completed += 1;
            }
            GameEvent::QuestFailed { .. } => {
                self.quests_failed += 1;
            }
            GameEvent::PlayerDied { .. } => {
                self.deaths += 1;
            }
            GameEvent::PerfectWord { .. } => {
                self.perfect_words += 1;
            }
            GameEvent::CombatEnded { outcome, .. } => {
                match outcome {
                    CombatOutcome::Victory { .. } => self.combats_won += 1,
                    CombatOutcome::Defeat => self.combats_lost += 1,
                    _ => {}
                }
            }
            GameEvent::FactionStandingChanged { .. } => {
                self.faction_changes += 1;
            }
            _ => {}
        }
    }
}

/// Event reactions - what systems should do when events occur
#[derive(Debug, Clone)]
pub struct EventReaction {
    pub trigger: EventTrigger,
    pub actions: Vec<ReactionAction>,
    pub priority: i32,
    pub one_shot: bool,
}

#[derive(Debug, Clone)]
pub enum EventTrigger {
    /// Triggered by event category
    Category(EventCategory),
    /// Triggered by specific faction event
    FactionEvent(Faction),
    /// Triggered by NPC event
    NPCEvent(String),
    /// Triggered by location
    LocationEvent(String),
    /// Custom predicate (evaluated at runtime)
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum ReactionAction {
    /// Emit another event
    EmitEvent(GameEvent),
    /// Emit event after delay
    EmitDelayed(GameEvent, u32),
    /// Log to narrative
    LogNarrative(String),
    /// Modify game state (stored as command string)
    ModifyState(String),
}

/// Process events and generate reactions
pub fn process_event_reactions(
    event: &GameEvent, 
    reactions: &[EventReaction]
) -> Vec<ReactionAction> {
    let mut results = Vec::new();
    
    for reaction in reactions {
        let triggered = match &reaction.trigger {
            EventTrigger::Category(cat) => event.category() == *cat,
            EventTrigger::FactionEvent(faction) => {
                matches!(event, 
                    GameEvent::FactionStandingChanged { faction: f, .. } |
                    GameEvent::FactionJoined { faction: f, .. } |
                    GameEvent::FactionPromoted { faction: f, .. }
                    if f == faction
                )
            }
            EventTrigger::NPCEvent(npc) => {
                matches!(event,
                    GameEvent::NPCMet { npc: n, .. } |
                    GameEvent::NPCDialogue { npc: n, .. }
                    if n == npc
                )
            }
            EventTrigger::LocationEvent(loc) => {
                matches!(event,
                    GameEvent::LocationEntered { location, .. } |
                    GameEvent::LocationDiscovered { location, .. }
                    if location == loc
                )
            }
            EventTrigger::Custom(_) => false, // Would need custom evaluation
        };
        
        if triggered {
            results.extend(reaction.actions.clone());
        }
    }
    
    results
}

/// Standard event reactions that should always be active
pub fn default_reactions() -> Vec<EventReaction> {
    vec![
        // When player dies, log it
        EventReaction {
            trigger: EventTrigger::Category(EventCategory::Player),
            actions: vec![
                ReactionAction::LogNarrative("The world grows darker...".to_string()),
            ],
            priority: 100,
            one_shot: false,
        },
        
        // Faction betrayal triggers bounty
        EventReaction {
            trigger: EventTrigger::Category(EventCategory::Faction),
            actions: vec![
                ReactionAction::LogNarrative("Your reputation precedes you.".to_string()),
            ],
            priority: 50,
            one_shot: false,
        },
    ]
}
