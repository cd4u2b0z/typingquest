//! TypingQuest - A Roguelike Typing Adventure
//! 
//! Inspired by ttyper, Undertale, Earthbound, and Balatro

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
use game::events::GameEvent;
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
    }

    Ok(())
}

enum InputResult {
    Continue,
    Quit,
}

fn handle_input(game: &mut GameState, key: KeyCode) -> InputResult {
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
    }
}

fn handle_title_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Up | KeyCode::Char('k') => game.move_menu_up(),
        KeyCode::Down | KeyCode::Char('j') => game.move_menu_down(3),
        KeyCode::Enter => {
            match game.menu_index {
                0 => {
                    // New Game
                    game.scene = Scene::ClassSelect;
                    game.menu_index = 0;
                }
                1 => {
                    // Continue (placeholder - would load save)
                    game.add_message("No save file found...");
                }
                2 => {
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
                    }
                    RoomType::Shop => {
                        game.enter_shop();
                    }
                    RoomType::Rest => {
                        game.enter_rest();
                    }
                    RoomType::Event => {
                        let event = GameEvent::random();
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
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                // Typing input
                combat.on_char_typed(c);
                
                // Check if word completed
                if combat.typed_input == combat.current_word {
                    game.total_words_typed += 1;
                    
                    // The combat system handles word completion internally
                    // Check if enemy defeated
                    if combat.enemy.current_hp <= 0 {
                        game.end_combat(true);
                        game.check_victory();
                    }
                }
            }
            KeyCode::Backspace => {
                combat.on_backspace();
            }
            _ => {}
        }
    }
    
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
            game.scene = Scene::Dungeon;
            game.menu_index = 0;
        }
        _ => {}
    }
    InputResult::Continue
}

fn handle_rest_input(game: &mut GameState, key: KeyCode) -> InputResult {
    match key {
        KeyCode::Up | KeyCode::Char('k') => game.move_menu_up(),
        KeyCode::Down | KeyCode::Char('j') => game.move_menu_down(3),
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
            game.scene = Scene::Dungeon;
            game.menu_index = 0;
        }
        KeyCode::Esc => {
            game.scene = Scene::Dungeon;
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
