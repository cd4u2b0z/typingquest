//! Game state management - the heart of the roguelike!

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
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
    voice_system::{FactionVoice, build_faction_voices, generate_faction_dialogue, DialogueContext},
    narrative::Faction,
    encounter_writing::{AuthoredEncounter, EncounterTracker, build_encounters},
    run_modifiers::{RunModifiers, RunType},
};
use crate::data::GameData;
use crate::ui::effects::EffectsManager;

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
    BattleSummary,
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
    /// Faction voice profiles for NPC dialogue
    pub faction_voices: HashMap<Faction, FactionVoice>,
    /// Current NPC dialogue (if any)
    pub current_npc_dialogue: Option<(String, String)>,
    /// Current battle summary (shown after combat)
    pub current_battle_summary: Option<crate::ui::stats_summary::BattleSummary>,
    /// All authored encounters
    pub encounters: HashMap<String, AuthoredEncounter>,
    /// Tracks which encounters have been seen/choices made
    pub encounter_tracker: EncounterTracker,
    /// Current authored encounter being displayed
    pub current_encounter: Option<AuthoredEncounter>,
    /// Run modifiers affecting difficulty/rewards
    pub run_modifiers: RunModifiers,
    /// Visual effects manager (floating text, screen shake, etc.)
    pub effects: EffectsManager,
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
            faction_voices: build_faction_voices(),
            current_npc_dialogue: None,
            current_battle_summary: None,
            encounters: build_encounters(),
            encounter_tracker: EncounterTracker::new(),
            current_encounter: None,
            run_modifiers: RunModifiers::new(),
            effects: EffectsManager::new(),
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
        self.combat_state = Some(CombatState::new(enemy, self.game_data.clone(), difficulty, difficulty, self.active_typing_modifier.clone(), Some(&self.skill_tree)));
        
        // Initialize immersion systems for this combat
        if let Some(ref mut combat) = self.combat_state {
            if let Some(ref player) = self.player {
                combat.init_immersion(&player.class);
            }
        }
        
        // Clear any lingering effects
        self.effects.clear();
        
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
                let xp_reward = ((enemy.xp_reward as f32) * self.skill_tree.get_xp_multiplier()).round() as u64;
                let gold_reward = ((enemy.gold_reward as f32) * self.run_modifiers.reward_multiplier).round() as u64;
                let is_boss = enemy.is_boss;
                
                // Create battle summary
                if let Some(combat) = &self.combat_state {
                    let summary = crate::ui::stats_summary::BattleSummary {
                        enemy_name: enemy_name.clone(),
                        victory: true,
                        was_boss: is_boss,
                        xp_gained: xp_reward as i32,
                        gold_gained: gold_reward as i32,
                        damage_dealt: combat.total_damage_dealt,
                        damage_taken: combat.total_damage_taken,
                        turns_taken: combat.turn,
                        words_completed: combat.turn,
                        max_combo: combat.max_combo,
                        accuracy: combat.correct_chars as f32 / combat.total_chars.max(1) as f32 * 100.0,
                        avg_wpm: if combat.wpm_samples.is_empty() { 0.0 } else { combat.wpm_samples.iter().sum::<f32>() / combat.wpm_samples.len() as f32 },
                        peak_wpm: combat.peak_wpm,
                        perfect_words: 0, // TODO: track perfect words
                        time_elapsed: combat.combat_start.elapsed().as_secs_f32(),
                    };
                    self.current_battle_summary = Some(summary);
                }
                
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
        // Transition to battle summary screen
        self.scene = Scene::BattleSummary;
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
        self.current_npc_dialogue = None;
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
        
        // Generate merchant greeting based on faction standing
        let greeting = self.get_merchant_greeting();
        self.current_npc_dialogue = Some(("Merchant".to_string(), greeting));
    }

    pub fn enter_rest(&mut self) {
        self.scene = Scene::Rest;
        self.menu_index = 0;
        
        // Generate Temple of Dawn greeting for rest sites
        let greeting = self.generate_npc_dialogue(Faction::TempleOfDawn, DialogueContext::Greeting);
        self.current_npc_dialogue = Some(("Healer".to_string(), greeting));
    }
    
    /// Generate faction-appropriate NPC dialogue
    pub fn generate_npc_dialogue(&self, faction: Faction, context: DialogueContext) -> String {
        let mut rng = rand::thread_rng();
        if let Some(voice) = self.faction_voices.get(&faction) {
            generate_faction_dialogue(voice, context, &mut rng)
        } else {
            "...".to_string()
        }
    }
    
    /// Get a greeting from a merchant based on faction standings
    pub fn get_merchant_greeting(&self) -> String {
        let mut rng = rand::thread_rng();
        
        // Merchant Consortium is the trading faction
        let faction = Faction::MerchantConsortium;
        let standing = self.faction_relations.standing(&faction);
        
        if let Some(voice) = self.faction_voices.get(&faction) {
            // Context depends on standing
            let context = if standing >= 50 {
                DialogueContext::Gratitude
            } else if standing <= -50 {
                DialogueContext::Warning
            } else {
                DialogueContext::Trading
            };
            generate_faction_dialogue(voice, context, &mut rng)
        } else {
            "Welcome to my shop, traveler.".to_string()
        }
    }
    
    /// Try to trigger an authored encounter for the current location
    pub fn try_trigger_encounter(&mut self) -> bool {
        let floor = self.get_current_floor();
        let location = format!("floor_{}", floor);
        
        // Find a valid encounter for this location
        let valid_encounter = self.encounters.values()
            .find(|e| {
                // Check location
                e.valid_locations.iter().any(|loc| loc == &location || loc == "any")
                // Check not already completed (unless repeatable)
                && (e.repeatable || !self.encounter_tracker.has_completed(&e.id))
                // Check chapter requirements
                && e.requirements.min_chapter.map_or(true, |min| floor >= min as i32)
                && e.requirements.max_chapter.map_or(true, |max| floor <= max as i32)
            })
            .cloned();
        
        if let Some(encounter) = valid_encounter {
            self.current_encounter = Some(encounter);
            return true;
        }
        false
    }
    
    /// Resolve an encounter choice
    pub fn resolve_encounter(&mut self, choice_idx: usize) {
        if let Some(encounter) = self.current_encounter.take() {
            if let Some(choice) = encounter.choices.get(choice_idx) {
                // Record the choice
                self.encounter_tracker.complete_encounter(&encounter.id, &choice.id);
                
                // Apply consequences
                let cons = &encounter.consequences;
                for (faction_name, change) in &cons.reputation_changes {
                    // Try to map faction name to enum
                    let faction: Option<Faction> = match faction_name.as_str() {
                        "MagesGuild" => Some(Faction::MagesGuild),
                        "TempleOfDawn" => Some(Faction::TempleOfDawn),
                        "ShadowGuild" => Some(Faction::ShadowGuild),
                        "MerchantConsortium" => Some(Faction::MerchantConsortium),
                        "RangersOfTheWild" => Some(Faction::RangersOfTheWild),
                        _ => None,
                    };
                    if let Some(f) = faction {
                        self.faction_relations.modify_standing(f, *change);
                    }
                }
                
                // Emit event
                self.event_bus.emit(BusEvent::RandomEncounter {
                    encounter_type: encounter.title.clone(),
                    location: format!("floor_{}", self.get_current_floor()),
                });
                
                self.add_message(&format!("Completed: {}", encounter.title));
            }
        }
    }


    
    /// Get enemy health multiplier from run modifiers
    pub fn get_enemy_health_multiplier(&self) -> f32 {
        use crate::game::run_modifiers::Modifier;
        let mut mult = 1.0;
        for active in &self.run_modifiers.active {
            if let Modifier::ToughEnemies { health_multiplier } = active.modifier {
                mult *= health_multiplier * active.level as f32;
            }
        }
        mult
    }
    
    /// Get enemy damage multiplier from run modifiers
    pub fn get_enemy_damage_multiplier(&self) -> f32 {
        use crate::game::run_modifiers::Modifier;
        let mut mult = 1.0;
        for active in &self.run_modifiers.active {
            if let Modifier::DangerousEnemies { damage_multiplier } = active.modifier {
                mult *= damage_multiplier * active.level as f32;
            }
        }
        mult
    }
    
    /// Get gold multiplier (reward_multiplier minus any drain)
    pub fn get_gold_multiplier(&self) -> f32 {
        use crate::game::run_modifiers::Modifier;
        let mut mult = self.run_modifiers.reward_multiplier;
        for active in &self.run_modifiers.active {
            if let Modifier::GoldDrain { reduction_percent } = active.modifier {
                mult *= 1.0 - (reduction_percent * active.level as f32);
            }
        }
        mult.max(0.1) // Minimum 10% gold
    }
    
    /// Set run type (applies preset modifiers)
    pub fn set_run_type(&mut self, run_type: RunType) {
        self.run_modifiers.set_run_type(run_type);
    }
    
    /// Get total heat level
    pub fn get_heat_level(&self) -> u32 {
        self.run_modifiers.total_heat
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

// ============================================================================
// Visual Effects Integration
// ============================================================================

impl GameState {
    /// Update visual effects each frame (call in main loop)
    pub fn update_effects(&mut self) {
        self.effects.update();
    }
    
    /// Trigger damage number and screen shake when player hits enemy
    pub fn effect_player_damage(&mut self, damage: i32, is_crit: bool) {
        self.effects.add_damage(damage, is_crit);
        
        // Bigger shake for crits
        if is_crit {
            self.effects.screen_shake = Some(crate::ui::effects::ScreenShake::medium());
            self.effects.hit_flash = Some(crate::ui::effects::HitFlash::critical());
        } else if damage > 20 {
            self.effects.screen_shake = Some(crate::ui::effects::ScreenShake::light());
        }
    }
    
    /// Trigger effects when player takes damage
    pub fn effect_enemy_damage(&mut self, damage: i32) {
        self.effects.player_hit(damage);
    }
    
    /// Trigger combo effects
    pub fn effect_combo(&mut self, combo: i32) {
        self.effects.add_combo(combo);
    }
    
    /// Trigger keystroke ripple effect
    pub fn effect_keystroke(&mut self, correct: bool) {
        self.effects.keystroke(correct);
    }
    
    /// Victory effects
    pub fn effect_victory(&mut self) {
        self.effects.floating_texts.push(
            crate::ui::effects::FloatingText::combo(999, 0.5, 0.3)
        );
        self.effects.combo_pulse = Some(crate::ui::effects::ComboPulse::new(999));
    }
    
    /// Defeat effects
    pub fn effect_defeat(&mut self) {
        self.effects.screen_shake = Some(crate::ui::effects::ScreenShake::heavy());
        self.effects.floating_texts.push(
            crate::ui::effects::FloatingText {
                text: "DEFEAT".to_string(),
                x: 0.5,
                y: 0.4,
                color: crate::ui::effects::TextColor::Miss,
                size: crate::ui::effects::TextSize::Huge,
                velocity_y: -0.5,
                opacity: 1.0,
                created_at: std::time::Instant::now(),
                lifetime_ms: 3000,
            }
        );
    }
    
    /// Perfect word typed effect
    pub fn effect_perfect(&mut self) {
        self.effects.floating_texts.push(
            crate::ui::effects::FloatingText::perfect(0.5, 0.5)
        );
    }
    
    /// Heal effect
    pub fn effect_heal(&mut self, amount: i32) {
        self.effects.floating_texts.push(
            crate::ui::effects::FloatingText {
                text: format!("+{}", amount),
                x: 0.5,
                y: 0.8,
                color: crate::ui::effects::TextColor::Heal,
                size: crate::ui::effects::TextSize::Large,
                velocity_y: -2.0,
                opacity: 1.0,
                created_at: std::time::Instant::now(),
                lifetime_ms: 1200,
            }
        );
    }
}
