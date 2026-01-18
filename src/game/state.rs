//! Game state management - the heart of the roguelike!

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::game::{
    player::Player,
    enemy::Enemy,
    combat::CombatState,
    dungeon::Dungeon,
    items::Item,
    events::GameEvent,
    help_system::{HelpSystem, HintManager},
    tutorial::{TutorialState, TutorialProgress},
    typing_feel::TypingFeel,
    faction_system::FactionRelations,
    meta_progression::MetaProgress,
    event_bus::{EventBus, GameEvent as BusEvent, CombatOutcome},
    narrative_seed::{NarrativeSeed, TypingModifier},
    skills::SkillTree,
};
use crate::data::GameData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scene {
    Title,
    Tutorial,
    ClassSelect,
    Dungeon,
    Combat,
    Shop,
    Rest,
    Event,
    Inventory,
    Stats,
    GameOver,
    Victory,
    /// Lore discovery popup
    Lore,
    /// Milestone/story event
    Milestone,
    /// Meta-progression upgrade shop
    Upgrades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuSelection {
    NewGame,
    Continue,
    Quit,
}

// Note: GameState isn't fully serializable due to CombatState containing Instant
// Save/load will need special handling
#[derive(Debug, Clone)]
pub struct GameState {
    pub scene: Scene,
    pub player: Option<Player>,
    pub dungeon: Option<Dungeon>,
    pub current_enemy: Option<Enemy>,
    pub combat_state: Option<CombatState>,
    pub current_event: Option<GameEvent>,
    pub shop_items: Vec<Item>,
    pub message_log: Vec<String>,
    pub menu_index: usize,
    pub runs_completed: i32,
    pub total_enemies_defeated: i32,
    pub total_words_typed: i32,
    pub best_wpm: f64,
    pub input_buffer: String,
    pub game_data: Arc<GameData>,
    pub help_system: HelpSystem,
    pub hint_manager: HintManager,
    pub tutorial_state: TutorialState,
    pub tutorial_progress: TutorialProgress,
    pub typing_feel: TypingFeel,
    /// Current lore discovery being viewed
    pub current_lore: Option<(String, String)>,
    /// Current milestone event being displayed  
    pub current_milestone: Option<String>,
    /// Discovered lore fragments (for journal)
    /// Floors whose milestones have been shown this run
    pub milestones_shown: std::collections::HashSet<u32>,
    pub discovered_lore: Vec<(String, String)>,
    /// Faction standings and relationships
    pub faction_relations: FactionRelations,
    /// Persistent meta-progression (survives death)
    pub meta_progress: MetaProgress,
    /// Meta-progression damage bonus (from unlocks)
    pub damage_bonus_percent: f32,
    /// Meta-progression time bonus (from unlocks)
    pub time_bonus_percent: f32,
    /// Central event bus for system communication
    pub event_bus: EventBus,
    /// Narrative seed for run coherence
    pub narrative_seed: Option<NarrativeSeed>,
    /// Active typing modifier from corruption
    pub active_typing_modifier: Option<TypingModifier>,
    /// Player skill tree
    pub skill_tree: SkillTree,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            scene: Scene::Title,
            player: None,
            dungeon: None,
            current_enemy: None,
            combat_state: None,
            current_event: None,
            shop_items: Vec::new(),
            message_log: Vec::new(),
            menu_index: 0,
            runs_completed: 0,
            total_enemies_defeated: 0,
            total_words_typed: 0,
            best_wpm: 0.0,
            input_buffer: String::new(),
            game_data: Arc::new(GameData::load_or_default()),
            help_system: HelpSystem::new(),
            hint_manager: HintManager::new(),
            tutorial_state: TutorialState::new(),
            tutorial_progress: TutorialProgress::load(),
            typing_feel: TypingFeel::new(),
            current_lore: None,
            current_milestone: None,
            milestones_shown: std::collections::HashSet::new(),
            discovered_lore: Vec::new(),
            faction_relations: FactionRelations::new(),
            meta_progress: MetaProgress::default(),
            damage_bonus_percent: 0.0,
            time_bonus_percent: 0.0,
            event_bus: EventBus::new(),
            narrative_seed: None,
            active_typing_modifier: None,
            skill_tree: SkillTree::new(),
        }
    }

    pub fn start_new_game(&mut self, mut player: Player) {
        // Apply meta-progression bonuses
        let bonus = self.meta_progress.start_run();
        player.max_hp += bonus.hp_bonus;
        player.hp += bonus.hp_bonus;
        player.gold += bonus.gold_bonus as u64;
        
        // Store bonuses for combat calculations
        self.damage_bonus_percent = bonus.damage_bonus_percent;
        self.time_bonus_percent = bonus.time_bonus_percent;
        
        self.player = Some(player);
        self.dungeon = Some(Dungeon::new());
        self.scene = Scene::Dungeon;
        self.message_log.clear();
        self.milestones_shown.clear();
        
        // Show bonus message if any
        if bonus.hp_bonus > 0 || bonus.gold_bonus > 0 {
            self.add_message(&format!("Meta-bonuses: +{} HP, +{} Gold", bonus.hp_bonus, bonus.gold_bonus));
        }
        self.add_message("Your typing quest begins!");
        
        // Generate narrative seed for this run
        let seed = NarrativeSeed::generate_random();
        self.active_typing_modifier = Some(seed.world_state.corruption_type.typing_modifier());
        
        // Emit run start event
        self.event_bus.emit(BusEvent::ChapterStarted {
            chapter: 1,
            title: format!("The {} begins", seed.world_state.corruption_type.name()),
        });
        
        // Show corruption warning
        self.add_message(&format!("󰈸 The {} corrupts this realm...", seed.world_state.corruption_type.name()));
        self.narrative_seed = Some(seed);
    }

    pub fn add_message(&mut self, msg: &str) {
        self.message_log.push(msg.to_string());
        // Keep only last 10 messages
        if self.message_log.len() > 10 {
            self.message_log.remove(0);
        }
    }

    pub fn start_combat(&mut self, enemy: Enemy) {
        let enemy_name = enemy.name.clone();
        let zone_name = self.dungeon.as_ref().map(|d| d.get_zone_name()).unwrap_or_else(|| "Unknown".to_string());
        
        self.current_enemy = Some(enemy.clone());
        let difficulty = self.dungeon.as_ref().map(|d| d.current_floor as u32).unwrap_or(1);
        self.combat_state = Some(CombatState::new(enemy, self.game_data.clone(), difficulty, difficulty, self.active_typing_modifier.clone()));
        self.scene = Scene::Combat;
        
        self.add_message(&format!("{} appears!", enemy_name));
        
        // Emit combat start event
        self.event_bus.emit(BusEvent::CombatStarted {
            enemy: enemy_name,
            location: zone_name,
        });
    }

    pub fn end_combat(&mut self, victory: bool) {
        if victory {
            if let Some(enemy) = &self.current_enemy {
                let enemy_name = enemy.name.clone();
                let xp_reward = enemy.xp_reward as u64;
                let gold_reward = enemy.gold_reward as u64;
                let is_boss = enemy.is_boss;
                
                self.add_message(&format!("Defeated {}!", enemy_name));
                
                if let Some(player) = &mut self.player {
                    player.gain_experience(xp_reward);
                    player.gold += gold_reward;
                }
                self.total_enemies_defeated += 1;
                
                // Emit combat victory event
                self.event_bus.emit(BusEvent::CombatEnded {
                    enemy: enemy_name.clone(),
                    outcome: CombatOutcome::Victory {
                        xp_gained: xp_reward as u32,
                        loot: vec![format!("{} gold", gold_reward)],
                    },
                });
                
                // Emit XP event
                self.event_bus.emit(BusEvent::ExperienceGained {
                    amount: xp_reward as u32,
                    source: enemy_name.clone(),
                });
                
                // Mark boss as defeated for this floor
                if is_boss {
                    if let Some(dungeon) = &mut self.dungeon {
                        dungeon.boss_defeated = true;
                        
                        // Final boss on floor 10 = victory!
                        if dungeon.current_floor >= 10 {
                            self.current_enemy = None;
                            self.combat_state = None;
                            self.scene = Scene::Victory;
                            self.runs_completed += 1;
                            return;
                        }
                    }
                }
            }
        }
        self.current_enemy = None;
        self.combat_state = None;
            
            // Mark current room as cleared and increment counter
            if let Some(dungeon) = &mut self.dungeon {
                dungeon.current_room.cleared = true;
                dungeon.rooms_cleared += 1;
            }
        self.scene = Scene::Dungeon;
    }

    pub fn start_event(&mut self, event: GameEvent) {
        self.current_event = Some(event);
        self.scene = Scene::Event;
    }

    pub fn end_event(&mut self) {
        self.current_event = None;
        self.scene = Scene::Dungeon;
        
        // Mark event room as cleared and increment counter
        if let Some(dungeon) = &mut self.dungeon {
            dungeon.current_room.cleared = true;
            dungeon.rooms_cleared += 1;
        }
    }
    pub fn end_rest(&mut self) {
        self.scene = Scene::Dungeon;
        
        // Check if floor is complete BEFORE incrementing (we're at the stairway)
        let should_advance = self.dungeon.as_ref().map(|d| d.floor_complete).unwrap_or(false);
        
        // Mark rest room as cleared and increment counter
        if let Some(dungeon) = &mut self.dungeon {
            dungeon.current_room.cleared = true;
            dungeon.rooms_cleared += 1;
            
            // If floor was complete, advance to next floor
            if should_advance {
                dungeon.advance_floor();
            }
        }
        
        // Show floor advancement message after dungeon borrow ends
        if should_advance {
            if let Some(dungeon) = &self.dungeon {
                self.add_message(&format!("Descended to floor {}!", dungeon.current_floor));
            }
        }
    }

    pub fn end_treasure(&mut self) {
        // Mark treasure room as cleared and increment counter
        if let Some(dungeon) = &mut self.dungeon {
            dungeon.current_room.cleared = true;
            dungeon.rooms_cleared += 1;
        }
    }

    pub fn end_shop(&mut self) {
        self.scene = Scene::Dungeon;
        self.shop_items.clear();
        
        // Mark shop room as cleared and increment counter
        if let Some(dungeon) = &mut self.dungeon {
            dungeon.current_room.cleared = true;
            dungeon.rooms_cleared += 1;
        }
    }


    pub fn enter_shop(&mut self) {
        use rand::seq::SliceRandom;
        
        let mut rng = rand::thread_rng();
        let mut items = Vec::new();
        
        // Add some consumables
        let consumables = Item::consumable_pool();
        for item in consumables.choose_multiple(&mut rng, 2) {
            items.push(item.clone());
        }
        
        // Add a joker if lucky
        if rand::random::<f32>() < 0.3 {
            let jokers = Item::joker_pool();
            if let Some(joker) = jokers.choose(&mut rng) {
                items.push(joker.clone());
            }
        }
        
        self.shop_items = items;
        self.scene = Scene::Shop;
        self.menu_index = 0;
    }

    pub fn enter_rest(&mut self) {
        self.scene = Scene::Rest;
        self.menu_index = 0;
    }

    pub fn check_game_over(&mut self) -> bool {
        if let Some(player) = &self.player {
            if player.hp <= 0 {
                // Award Ink based on progress
                let floor = self.get_current_floor() as u64;
                let ink_earned = floor * 10 + (self.total_enemies_defeated as u64 * 2) 
                    + (self.total_words_typed as u64);
                self.meta_progress.current_ink += ink_earned;
                self.meta_progress.total_ink += ink_earned;
                self.meta_progress.runs_attempted += 1;
                self.add_message(&format!("󰙤 Earned {} Ink from this run", ink_earned));
                
                self.scene = Scene::GameOver;
                return true;
            }
        }
        false
    }

    pub fn check_victory(&mut self) -> bool {
        if let Some(dungeon) = &self.dungeon {
            if dungeon.current_floor > 10 {
                self.scene = Scene::Victory;
                self.runs_completed += 1;
                return true;
            }
        }
        false
    }

    pub fn get_current_floor(&self) -> i32 {
        self.dungeon.as_ref().map(|d| d.current_floor).unwrap_or(1)
    }

    pub fn move_menu_up(&mut self) {
        if self.menu_index > 0 {
            self.menu_index -= 1;
        }
    }

    pub fn move_menu_down(&mut self, max: usize) {
        if self.menu_index < max.saturating_sub(1) {
            self.menu_index += 1;
        }
    }
    
    /// Process all pending events from the event bus
    /// This is the "nervous system" - where systems react to each other
    pub fn process_events(&mut self) {
        // Tick deferred events
        self.event_bus.tick();
        
        // Process all pending events
        let events = self.event_bus.drain_all();
        for event in events {
            self.handle_event(event);
        }
    }
    
    /// Handle a single game event - triggers reactions across systems
    fn handle_event(&mut self, event: BusEvent) {
        match &event {
            BusEvent::CombatEnded { enemy, outcome } => {
                // Update faction relations based on combat
                if let CombatOutcome::Victory { .. } = outcome {
                    // Defeating enemies improves reputation with most factions
                    self.faction_relations.modify_all_standings(1, &format!("Defeated {}", enemy));
                }
            }
            BusEvent::ExperienceGained { amount, source } => {
                // Track XP for statistics
                if let Some(player) = &self.player {
                    self.add_message(&format!("Gained {} XP from {}", amount, source));
                }
            }
            BusEvent::LoreDiscovered { lore_id, category } => {
                // Could trigger UI notification, journal update, etc.
                self.add_message(&format!("Discovered lore: {} ({})", lore_id, category));
            }
            BusEvent::FactionStandingChanged { faction, old_standing, new_standing, reason } => {
                let direction = if new_standing > old_standing { "improved" } else { "worsened" };
                self.add_message(&format!("Your standing with {:?} has {} ({})", faction, direction, reason));
            }
            BusEvent::PlayerLeveledUp { new_level, skill_points } => {
                self.add_message(&format!("Level up! Now level {} (+{} skill points)", new_level, skill_points));
            }
            // Add more event handlers as systems get wired up
            _ => {
                // Log unhandled events for debugging if needed
            }
        }
    }
}
