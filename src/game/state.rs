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
    pub discovered_lore: Vec<(String, String)>,
    /// Faction standings and relationships
    pub faction_relations: FactionRelations,
    /// Persistent meta-progression (survives death)
    pub meta_progress: MetaProgress,
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
            discovered_lore: Vec::new(),
            faction_relations: FactionRelations::new(),
            meta_progress: MetaProgress::default(),
        }
    }

    pub fn start_new_game(&mut self, player: Player) {
        self.player = Some(player);
        self.dungeon = Some(Dungeon::new());
        self.scene = Scene::Dungeon;
        self.message_log.clear();
        self.add_message("Your typing quest begins!");
    }

    pub fn add_message(&mut self, msg: &str) {
        self.message_log.push(msg.to_string());
        // Keep only last 10 messages
        if self.message_log.len() > 10 {
            self.message_log.remove(0);
        }
    }

    pub fn start_combat(&mut self, enemy: Enemy) {
        self.current_enemy = Some(enemy.clone());
        let difficulty = self.dungeon.as_ref().map(|d| d.current_floor as u32).unwrap_or(1);
        self.combat_state = Some(CombatState::new(enemy, self.game_data.clone(), difficulty));
        self.scene = Scene::Combat;
        if let Some(e) = &self.current_enemy {
            self.add_message(&format!("{} appears!", e.name));
        }
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
}
