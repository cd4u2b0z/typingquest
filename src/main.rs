//! Keyboard Warrior - A Roguelike Typing Adventure
//! 
//! Inspired by ttyper, Undertale, Earthbound, and Balatro
//!
//! 󰩛 Original work by Dr. Baklava 󰩛

mod game;
mod data;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use game::state::{GameState, Scene};
use game::player::{Player, Class};
use game::enemy::Enemy;
use game::world_integration::{get_floor_milestone, generate_zone_event, FloorZone};
use game::dungeon::RoomType;
use game::combat::CombatPhase;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup better panic messages for debugging
    better_panic::install();
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create game state
    let mut game = GameState::new();

    // Main game loop
    let result = run_game(&mut terminal, &mut game);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_game(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    game: &mut GameState,
) -> Result<(), Box<dyn std::error::Error>> {
    let tick_rate = Duration::from_millis(50);

    loop {
        // Render
        terminal.draw(|f| ui::render::render(f, game))?;

        // Handle input
        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match handle_input(game, key.code) {
                        InputResult::Quit => break,
                        InputResult::Continue => {}
                    }
                }
            }
        }

        // Update combat timer if in combat
        if let Some(combat) = &mut game.combat_state {
            combat.tick();
            
            // Check for time running out OR enemy turn phase
            if combat.time_remaining <= 0.0 || combat.phase == CombatPhase::EnemyTurn {
                // Enemy attacks
                if let Some(player) = &mut game.player {
                    combat.execute_enemy_turn(player);
                            // Player took damage - trigger feel effects
                            game.typing_feel.screen_shake = 0.5;
                }
            }
            
            // Check for combat ending
            if combat.phase == CombatPhase::Victory {
                game.end_combat(true);
                game.check_victory();
            } else if combat.phase == CombatPhase::Defeat {
                game.check_game_over();
            }
        }
        
        // Process events from the event bus (system reactions)
        game.process_events();
    }

    Ok(())
}

enum InputResult {
    Continue,
    Quit,
}

fn handle_input(game: &mut GameState, key: KeyCode) -> InputResult {
    // Update help system context
    game.help_system.update_context(game.scene);
    
    // Help overlay intercepts input when visible
    if game.help_system.visible {
        return handle_help_input(game, key);
    }
    
    // Global help toggle (? only during combat/tutorial, h elsewhere)
    // During combat/tutorial, 'h' should go to typing, not help
    let in_typing_mode = matches!(game.scene, Scene::Combat | Scene::Tutorial);
    match key {
        KeyCode::Char('?') => {
            game.help_system.toggle();
            return InputResult::Continue;
        }
        KeyCode::Char('h') if !in_typing_mode => {
            game.help_system.toggle();
            return InputResult::Continue;
        }
        _ => {}
    }
    
    match game.scene {
        Scene::Title => handle_title_input(game, key),
        Scene::ClassSelect => handle_class_select_input(game, key),
        Scene::Dungeon => handle_dungeon_input(game, key),
        Scene::Combat => handle_combat_input(game, key),
        Scene::Shop => handle_shop_input(game, key),
        Scene::Rest => handle_rest_input(game, key),
        Scene::Event => handle_event_input(game, key),
        Scene::Inventory => handle_inventory_input(game, key),
        Scene::Stats => handle_stats_input(game, key),
        Scene::GameOver => handle_game_over_input(game, key),
        Scene::Victory => handle_victory_input(game, key),
        Scene::Tutorial => handle_tutorial_input(game, key),
        Scene::Lore => handle_lore_input(game, key),
        Scene::Milestone => handle_milestone_input(game, key),
        Scene::Upgrades => handle_upgrades_input(game, key),
    }
}

/// Handle input when help overlay is open
fn handle_help_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        // Close help
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('h') => {
            game.help_system.hide();
        }
        // Tab navigation
        KeyCode::Tab => {
            game.help_system.next_tab();
        }
        KeyCode::BackTab => {
            game.help_system.prev_tab();
        }
        // Number keys for tabs
        KeyCode::Char('1') => game.help_system.select_tab(1),
        KeyCode::Char('2') => game.help_system.select_tab(2),
        KeyCode::Char('3') => game.help_system.select_tab(3),
        KeyCode::Char('4') => game.help_system.select_tab(4),
        // Scrolling
        KeyCode::Down | KeyCode::Char('j') => {
            game.help_system.scroll_down();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            game.help_system.scroll_up();
        }
        // Quit still works
        KeyCode::Char('q') => return InputResult::Quit,
        _ => {}
    }
    InputResult::Continue
}

fn handle_title_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Up | KeyCode::Char('k') => game.move_menu_up(),
        KeyCode::Down | KeyCode::Char('j') => game.move_menu_down(5), // Now 5 items
        KeyCode::Enter => {
            match game.menu_index {
                0 => {
                    // New Game
                    game.scene = Scene::ClassSelect;
                    game.menu_index = 0;
                }
                1 => {
                    // Tutorial
                    game.tutorial_state.reset();
                    game.scene = Scene::Tutorial;
                }
                2 => {
                    // Upgrades (meta-progression shop)
                    game.scene = Scene::Upgrades;
                    game.menu_index = 0;
                }
                3 => {
                    // Continue (placeholder - would load save)
                    game.add_message("No save file found...");
                }
                4 => {
                    // Quit
                    return InputResult::Quit;
                }
                _ => {}
            }
        }
        KeyCode::Char('n') => {
            game.scene = Scene::ClassSelect;
            game.menu_index = 0;
        }
        KeyCode::Char('u') => {
            game.scene = Scene::Upgrades;
            game.menu_index = 0;
        }
        KeyCode::Char('q') => return InputResult::Quit,
        _ => {}
    }
    InputResult::Continue
}

fn handle_class_select_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Up | KeyCode::Char('k') => game.move_menu_up(),
        KeyCode::Down | KeyCode::Char('j') => game.move_menu_down(5),
        KeyCode::Enter => {
            let class = match game.menu_index {
                0 => Class::Wordsmith,
                1 => Class::Scribe,
                2 => Class::Spellweaver,
                3 => Class::Barbarian,
                4 => Class::Trickster,
                _ => Class::Wordsmith,
            };
            let player = Player::new("Hero".to_string(), class);
            game.start_new_game(player);
        }
        KeyCode::Esc => {
            game.scene = Scene::Title;
            game.menu_index = 0;
        }
        _ => {}
    }
    InputResult::Continue
}

fn handle_dungeon_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Char('e') | KeyCode::Enter => {
            // First check for pending lore discovery from previous room
            if let Some(dungeon) = &game.dungeon {
                if let Some(lore) = dungeon.pending_lore.clone() {
                    if let Some(d) = &mut game.dungeon {
                        d.pending_lore = None;
                    }
                    game.current_lore = Some(lore);
                    game.scene = Scene::Lore;
                    return InputResult::Continue;
                }
            }
            
            // Check for milestone events at special floors
            let floor = game.get_current_floor();
            if let Some(milestone) = get_floor_milestone(floor as u32) {
                // Only show milestone once per floor (on first room) and if not already shown
                if !game.milestones_shown.contains(&(floor as u32)) {
                    if let Some(dungeon) = &game.dungeon {
                        if dungeon.rooms_cleared == 0 && dungeon.current_room.room_type == RoomType::Start {
                            game.milestones_shown.insert(floor as u32);
                            game.current_milestone = Some(milestone.description);
                            game.scene = Scene::Milestone;
                            return InputResult::Continue;
                        }
                    }
                }
            }

            // Explore - go to next room
            if let Some(dungeon) = &mut game.dungeon {
                let room = dungeon.generate_next_room();
                match room.room_type {
                    RoomType::Start => {
                        // Starting room - just a message
                        game.add_message("You enter the dungeon...");
                    }
                    RoomType::Combat => {
                        let floor = game.get_current_floor();
                        let enemy = Enemy::random_for_floor(floor);
                        game.start_combat(enemy);
                    }
                    RoomType::Elite => {
                        let floor = game.get_current_floor();
                        let enemy = Enemy::random_elite(floor);
                        game.start_combat(enemy);
                    }
                    RoomType::Boss => {
                        let floor = game.get_current_floor();
                        let enemy = Enemy::random_boss(floor);
                        game.start_combat(enemy);
                    }
                    RoomType::Treasure => {
                        // Give random item
                        let item = game::items::Item::random_consumable();
                        if let Some(player) = &mut game.player {
                            player.inventory.push(item.clone());
                            game.add_message(&format!("Found {}!", item.name));
                        }
                        game.end_treasure();
                    }
                    RoomType::Shop => {
                        game.enter_shop();
                    }
                    RoomType::Rest => {
                        game.enter_rest();
                    }
                    RoomType::Event => {
                        // Use zone-specific events for more variety
                        let floor = game.get_current_floor();
                        let zone = FloorZone::from_floor(floor as u32);
                        let event = generate_zone_event(zone);
                        game.start_event(event);
                    }
                }
            }
        }
        KeyCode::Char('i') => {
            game.scene = Scene::Inventory;
            game.menu_index = 0;
        }
        KeyCode::Char('s') => {
            game.scene = Scene::Stats;
        }
        KeyCode::Char('q') => return InputResult::Quit,
        _ => {}
    }
    InputResult::Continue
}

fn handle_combat_input(game: &mut GameState, key: KeyCode) -> InputResult {
    if let Some(combat) = &mut game.combat_state {
        match key {
            // Tab toggles spell mode
            KeyCode::Tab => {
                combat.toggle_spell_mode();
                if combat.spell_mode {
                    game.add_message("󰊠 SPELL MODE - Press 1-9 to select a spell, Tab to cancel");
                } else {
                    game.add_message("Normal attack mode");
                }
            }
            // Number keys select spells when in spell mode
            KeyCode::Char(n) if combat.spell_mode && n.is_ascii_digit() && n != '0' => {
                let spell_idx = (n as u8 - b'1') as usize;
                if let Some(player) = &game.player {
                    if spell_idx < player.known_spells.len() {
                        let spell = player.known_spells[spell_idx].clone();
                        if let Some(combat) = &mut game.combat_state {
                            combat.select_spell(&spell);
                        }
                    } else {
                        game.add_message("No spell in that slot!");
                    }
                }
            }
            KeyCode::Esc => {
                // Flee attempt
                if combat.try_flee() {
                    game.add_message("You fled successfully!");
                    game.combat_state = None;
                    game.current_enemy = None;
                    game.scene = Scene::Dungeon;
                } else {
                    game.add_message("Couldn't escape!");
                    if let Some(player) = &mut game.player {
                        if let Some(combat) = &mut game.combat_state {
                            combat.execute_enemy_turn(player);
                            // Player took damage - trigger feel effects
                            game.typing_feel.screen_shake = 0.5;
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                // Track state before typing for typing_feel updates
                let word_before = combat.current_word.clone();
                let typed_len_before = combat.typed_input.len();
                let word_was_complete = combat.typed_input == combat.current_word;
                
                // Typing input
                combat.on_char_typed(c);
                
                // Update typing feel system
                let typed_len_after = combat.typed_input.len();
                if typed_len_after > typed_len_before {
                    // A character was accepted
                    let char_index = typed_len_after - 1;
                    let expected = word_before.chars().nth(char_index).unwrap_or(' ');
                    let is_correct = c == expected;
                    game.typing_feel.on_keystroke(is_correct, char_index, expected, c);
                }
                
                // Check if word completed
                if combat.typed_input == combat.current_word && !word_was_complete {
                    game.total_words_typed += 1;
                    
                    // Update typing feel with word completion
                    let time_taken = combat.time_limit - combat.time_remaining;
                    game.typing_feel.on_word_complete(&word_before, &combat.typed_input, time_taken);
                    
                    // Sync combo from typing_feel back to combat for display
                    combat.combo = game.typing_feel.combo;
                    if combat.combo > combat.max_combo {
                        combat.max_combo = combat.combo;
                    }
                    
                    // Handle spell casting if in spell mode
                    if combat.spell_mode {
                        if let Some(incantation) = &combat.spell_incantation.clone() {
                            // Find the spell that matches
                            if let Some(player) = &mut game.player {
                                if let Some(spell) = player.known_spells.iter().find(|s| &s.incantation == incantation).cloned() {
                                    if let Some(combat) = &mut game.combat_state {
                                        combat.cast_spell(&spell, player);
                                    }
                                }
                            }
                        }
                    }
                    
                    // Check if enemy defeated
                    if let Some(combat) = &game.combat_state {
                        if combat.enemy.current_hp <= 0 {
                            game.end_combat(true);
                            game.check_victory();
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                combat.on_backspace();
            }
            _ => {}
        }
    }
    
    // Update typing feel effects
    game.typing_feel.tick(0.016);
    // Update typing feel effects
    game.typing_feel.tick(0.016);
    // Check for player death
    game.check_game_over();
    
    InputResult::Continue
}

fn handle_shop_input(game: &mut GameState, key: KeyCode) -> InputResult {
    let item_count = game.shop_items.len();
    match key {
        KeyCode::Up | KeyCode::Char('k') => game.move_menu_up(),
        KeyCode::Down | KeyCode::Char('j') => game.move_menu_down(item_count),
        KeyCode::Enter => {
            if game.menu_index < game.shop_items.len() {
                let item = game.shop_items[game.menu_index].clone();
                if let Some(player) = &mut game.player {
                    if player.gold >= item.price as u64 {
                        player.gold -= item.price as u64;
                        player.inventory.push(item.clone());
                        game.add_message(&format!("Bought {}!", item.name));
                        game.shop_items.remove(game.menu_index);
                        if game.menu_index > 0 && game.menu_index >= game.shop_items.len() {
                            game.menu_index = game.shop_items.len().saturating_sub(1);
                        }
                    } else {
                        game.add_message("Not enough gold!");
                    }
                }
            }
        }
        KeyCode::Esc => {
            game.end_shop();
            game.menu_index = 0;
        }
        _ => {}
    }
    InputResult::Continue
}

fn handle_rest_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Up | KeyCode::Char('k') => game.move_menu_up(),
        KeyCode::Down | KeyCode::Char('j') => game.move_menu_down(4),
        KeyCode::Enter | KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') => {
            let choice = match key {
                KeyCode::Char('1') => 0,
                KeyCode::Char('2') => 1,
                KeyCode::Char('3') => 2,
                _ => game.menu_index,
            };
            
            if let Some(player) = &mut game.player {
                match choice {
                    0 => {
                        // Rest - heal 30% HP
                        let heal_amount = (player.max_hp as f32 * 0.3) as i32;
                        player.heal(heal_amount);
                        game.add_message(&format!("Rested and recovered {} HP!", heal_amount));
                    }
                    1 => {
                        // Train - gain XP
                        let xp = 20 + (player.level * 5) as u64;
                        player.gain_experience(xp);
                        game.add_message(&format!("Training complete! Gained {} XP.", xp));
                    }
                    2 => {
                        // Meditate - restore 50% MP
                        let restore = (player.max_mp as f32 * 0.5) as i32;
                        player.restore_mp(restore);
                        game.add_message(&format!("Meditation complete! Restored {} MP.", restore));
                    }
                    _ => {}
                }
            }
            game.end_rest();
            game.menu_index = 0;
        }
        KeyCode::Esc => {
            game.end_rest();
            game.menu_index = 0;
        }
        _ => {}
    }
    InputResult::Continue
}

fn handle_event_input(game: &mut GameState, key: KeyCode) -> InputResult {
    let choice_count = game.current_event.as_ref().map(|e| e.choices.len()).unwrap_or(0);
    
    match key {
        KeyCode::Up | KeyCode::Char('k') => game.move_menu_up(),
        KeyCode::Down | KeyCode::Char('j') => game.move_menu_down(choice_count),
        KeyCode::Enter | KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('3') => {
            let choice_idx = match key {
                KeyCode::Char('1') => 0,
                KeyCode::Char('2') => 1,
                KeyCode::Char('3') => 2,
                _ => game.menu_index,
            };
            
            if let Some(event) = &game.current_event {
                if choice_idx < event.choices.len() {
                    let outcome = event.choices[choice_idx].outcome.clone();
                    apply_event_outcome(game, outcome);
                }
            }
            game.end_event();
        }
        _ => {}
    }
    InputResult::Continue
}

fn apply_event_outcome(game: &mut GameState, outcome: game::events::EventOutcome) {
    use game::events::EventOutcome;
    
    if let Some(player) = &mut game.player {
        match outcome {
            EventOutcome::GainGold(amount) => {
                player.gold += amount as u64;
                game.add_message(&format!("Gained {} gold!", amount));
            }
            EventOutcome::LoseGold(amount) => {
                player.gold = player.gold.saturating_sub(amount as u64);
                game.add_message(&format!("Lost {} gold!", amount));
            }
            EventOutcome::GainHP(amount) => {
                player.heal(amount);
                game.add_message(&format!("Restored {} HP!", amount));
            }
            EventOutcome::LoseHP(amount) => {
                player.take_damage(amount);
                game.add_message(&format!("Lost {} HP!", amount));
            }
            EventOutcome::GainXP(amount) => {
                player.gain_experience(amount as u64);
                game.add_message(&format!("Gained {} XP!", amount));
            }
            EventOutcome::GainMaxHP(amount) => {
                player.max_hp += amount;
                player.hp += amount;
                game.add_message(&format!("Max HP increased by {}!", amount));
            }
            EventOutcome::GainItem => {
                let item = game::items::Item::random_consumable();
                player.inventory.push(item.clone());
                game.add_message(&format!("Found {}!", item.name));
            }
            EventOutcome::Nothing => {
                game.add_message("Nothing happened...");
            }
            EventOutcome::Combat => {
                let floor = game.get_current_floor();
                let enemy = Enemy::random_for_floor(floor);
                game.start_combat(enemy);
            }
            EventOutcome::FactionRep(faction, amount) => {
                game.faction_relations.modify_standing(faction, amount);
                let status = game.faction_relations.status(&faction);
                if amount > 0 {
                    game.add_message(&format!("󰜃 {} reputation with {:?}: {:?}", 
                        if amount >= 10 { "Major gain" } else { "Gained" }, faction, status));
                } else {
                    game.add_message(&format!("󰜃 {} reputation with {:?}: {:?}", 
                        if amount <= -10 { "Major loss" } else { "Lost" }, faction, status));
                }
            }
        }
    }
}

fn handle_inventory_input(game: &mut GameState, key: KeyCode) -> InputResult {
    let inv_size = game.player.as_ref().map(|p| p.inventory.len()).unwrap_or(0);
    
    match key {
        KeyCode::Up | KeyCode::Char('k') => game.move_menu_up(),
        KeyCode::Down | KeyCode::Char('j') => game.move_menu_down(inv_size),
        KeyCode::Enter => {
            let mut message = None;
            let mut new_menu_index = None;
            
            if let Some(player) = &mut game.player {
                if game.menu_index < player.inventory.len() {
                    let item = player.inventory.remove(game.menu_index);
                    // Apply item effect
                    match &item.effect {
                        game::items::ItemEffect::HealHP(amount) => {
                            player.heal(*amount);
                            message = Some(format!("Used {}! Restored {} HP.", item.name, amount));
                        }
                        game::items::ItemEffect::HealMP(amount) => {
                            player.restore_mp(*amount);
                            message = Some(format!("Used {}! Restored {} MP.", item.name, amount));
                        }
                        _ => {
                            message = Some(format!("Used {}!", item.name));
                        }
                    }
                    if game.menu_index > 0 && game.menu_index >= player.inventory.len() {
                        new_menu_index = Some(player.inventory.len().saturating_sub(1));
                    }
                }
            }
            
            if let Some(msg) = message {
                game.add_message(&msg);
            }
            if let Some(idx) = new_menu_index {
                game.menu_index = idx;
            }
        }
        KeyCode::Esc => {
            game.scene = Scene::Dungeon;
            game.menu_index = 0;
        }
        _ => {}
    }
    InputResult::Continue
}

fn handle_stats_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
            game.scene = Scene::Dungeon;
        }
        _ => {}
    }
    InputResult::Continue
}

fn handle_game_over_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Char('r') => {
            // Restart
            *game = GameState::new();
            game.scene = Scene::ClassSelect;
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            return InputResult::Quit;
        }
        _ => {}
    }
    InputResult::Continue
}

fn handle_victory_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Char('n') => {
            // New Game+
            *game = GameState::new();
            game.scene = Scene::ClassSelect;
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            return InputResult::Quit;
        }
        _ => {}
    }
    InputResult::Continue
}

/// Handle lore discovery popup - any key dismisses
fn handle_lore_input(game: &mut GameState, _key: KeyCode) -> InputResult {
    // Save the lore to discovered list
    if let Some(lore) = game.current_lore.take() {
        game.discovered_lore.push(lore);
    }
    game.scene = Scene::Dungeon;
    InputResult::Continue
}

/// Handle milestone event - Enter to continue
fn handle_milestone_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Enter => {
            game.current_milestone = None;
            game.scene = Scene::Dungeon;
        }
        _ => {}
    }
    InputResult::Continue
}

/// Handle input in the upgrades/meta-progression shop
fn handle_upgrades_input(game: &mut GameState, key: KeyCode) -> InputResult {
    let upgrades = game.meta_progress.get_available_upgrades();
    let max_index = upgrades.len().saturating_sub(1);
    
    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            if game.menu_index > 0 {
                game.menu_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if game.menu_index < max_index {
                game.menu_index += 1;
            }
        }
        KeyCode::Enter => {
            // Try to purchase selected upgrade
            if let Some(upgrade) = upgrades.get(game.menu_index) {
                match game.meta_progress.purchase_upgrade(&upgrade.id) {
                    Ok(()) => {
                        game.add_message(&format!("Purchased {}!", upgrade.name));
                    }
                    Err(e) => {
                        game.add_message(&format!("Cannot purchase: {}", e));
                    }
                }
            }
        }
        KeyCode::Esc => {
            game.scene = Scene::Title;
            game.menu_index = 0;
        }
        _ => {}
    }
    InputResult::Continue
}

fn handle_tutorial_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Esc => {
            // Exit tutorial, go back to title
            game.scene = Scene::Title;
            game.menu_index = 0;
        }
        KeyCode::Enter => {
            // Advance to next step or complete tutorial
            if game.tutorial_state.is_step_complete() {
                let completed = game.tutorial_state.advance();
                if completed {
                    // Tutorial complete, save progress and start the game
                    game.tutorial_progress.mark_completed();
                    game.tutorial_progress.save();
                    game.scene = Scene::ClassSelect;
                    game.menu_index = 0;
                }
            }
        }
        KeyCode::Tab => {
            // Skip current step (for experienced players)
            let completed = game.tutorial_state.advance();
            if completed {
                game.tutorial_progress.mark_completed();
                game.tutorial_progress.save();
                game.scene = Scene::ClassSelect;
                game.menu_index = 0;
            }
        }
        KeyCode::Char(c) => {
            // Type characters for the tutorial
            game.tutorial_state.type_char(c);
        }
        KeyCode::Backspace => {
            game.tutorial_state.backspace();
        }
        _ => {}
    }
    InputResult::Continue
}
