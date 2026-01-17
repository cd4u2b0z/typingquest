//! Terminal UI rendering using ratatui

use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::game::state::{GameState, Scene};
use crate::game::combat::CombatPhase;

pub fn render(f: &mut Frame, state: &GameState) {
    match state.scene {
        Scene::Title => render_title(f, state),
        Scene::ClassSelect => render_class_select(f, state),
        Scene::Dungeon => render_dungeon(f, state),
        Scene::Combat => render_combat(f, state),
        Scene::Shop => render_shop(f, state),
        Scene::Rest => render_rest(f, state),
        Scene::Event => render_event(f, state),
        Scene::Inventory => render_inventory(f, state),
        Scene::Stats => render_stats(f, state),
        Scene::GameOver => render_game_over(f, state),
        Scene::Victory => render_victory(f, state),
    }
}

fn render_title(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(12),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(f.area());

    // ASCII art title
    let title_art = r#"
╔════════════════════════════════════════════════════════════════╗
║  ████████╗██╗   ██╗██████╗ ██╗███╗   ██╗ ██████╗               ║
║  ╚══██╔══╝╚██╗ ██╔╝██╔══██╗██║████╗  ██║██╔════╝               ║
║     ██║    ╚████╔╝ ██████╔╝██║██╔██╗ ██║██║  ███╗              ║
║     ██║     ╚██╔╝  ██╔═══╝ ██║██║╚██╗██║██║   ██║              ║
║     ██║      ██║   ██║     ██║██║ ╚████║╚██████╔╝              ║
║     ╚═╝      ╚═╝   ╚═╝     ╚═╝╚═╝  ╚═══╝ ╚═════╝  QUEST        ║
╚════════════════════════════════════════════════════════════════╝"#;

    let title = Paragraph::new(title_art)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Subtitle
    let subtitle = Paragraph::new("A Roguelike Typing Adventure")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center);
    f.render_widget(subtitle, chunks[1]);

    // Menu
    let menu_items = vec!["[N] New Game", "[C] Continue", "[Q] Quit"];
    let menu: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == state.menu_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*item).style(style)
        })
        .collect();

    let menu_widget = List::new(menu)
        .block(Block::default().borders(Borders::ALL).title(" Menu "));
    f.render_widget(menu_widget, chunks[2]);
}

fn render_class_select(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("Choose Your Class")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let classes = vec![
        ("Wordsmith", "Balanced fighter. +10% damage, starts with Heal spell.", Color::White),
        ("Scribe", "High MP, spell specialist. +25% MP, learns spells faster.", Color::Blue),
        ("Spellweaver", "Glass cannon mage. +50% spell damage, -20% HP.", Color::Magenta),
        ("Barbarian", "Tank with raw power. +30% HP, +15% damage, no spells.", Color::Red),
        ("Trickster", "Luck-based chaos. Random bonuses, critical hits, steals.", Color::Green),
    ];

    let class_items: Vec<ListItem> = classes
        .iter()
        .enumerate()
        .map(|(i, (name, desc, color))| {
            let style = if i == state.menu_index {
                Style::default().fg(*color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(*color)
            };
            let content = format!("{}: {}", name, desc);
            ListItem::new(content).style(style)
        })
        .collect();

    let class_list = List::new(class_items)
        .block(Block::default().borders(Borders::ALL).title(" Classes "));
    f.render_widget(class_list, chunks[1]);

    let help = Paragraph::new("↑/↓ to select, Enter to confirm, Esc to go back")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn render_dungeon(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(f.area());

    // Header with floor info
    let floor = state.get_current_floor();
    let header = Paragraph::new(format!("Floor {} - The Depths of QWERTY", floor))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Player stats
    if let Some(player) = &state.player {
        let hp_percent = (player.hp as f64 / player.max_hp as f64 * 100.0) as u16;
        let _mp_percent = (player.mp as f64 / player.max_mp as f64 * 100.0) as u16;
        
        let stats_text = format!(
            "HP: {}/{} | MP: {}/{} | Lv.{} | Gold: {} | XP: {}/{}",
            player.hp, player.max_hp,
            player.mp, player.max_mp,
            player.level, player.gold,
            player.experience, player.experience_to_next_level()
        );
        
        let stats = Paragraph::new(stats_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(format!(" {} - {} ", player.name, player.class.name())));
        f.render_widget(stats, chunks[1]);
    }

    // Room display / map
    if let Some(dungeon) = &state.dungeon {
        let room_display = dungeon.get_ascii_map();
        let room = Paragraph::new(room_display)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" Dungeon Map "));
        f.render_widget(room, chunks[2]);
    }

    // Actions
    let actions = Paragraph::new("[E] Explore  [I] Inventory  [S] Stats  [R] Rest (if available)  [Q] Quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" Actions "));
    f.render_widget(actions, chunks[3]);
}

fn render_combat(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(8),  // Enemy display
            Constraint::Length(3),  // Enemy HP bar
            Constraint::Min(6),     // Typing area (larger for sentences)
            Constraint::Length(3),  // Player HP
            Constraint::Length(6),  // Battle log
            Constraint::Length(2),  // Help
        ])
        .split(f.area());

    if let (Some(combat), Some(enemy)) = (&state.combat_state, &state.current_enemy) {
        // Enemy ASCII art and name
        let enemy_display = format!(
            "{}\n\n{}\n{}",
            enemy.ascii_art,
            enemy.name,
            enemy.battle_cry
        );
        let enemy_widget = Paragraph::new(enemy_display)
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(enemy_widget, chunks[0]);

        // Enemy HP bar
        let hp_percent = ((combat.enemy.current_hp as f64 / combat.enemy.max_hp as f64) * 100.0) as u16;
        let hp_color = if hp_percent > 50 { Color::Green } else if hp_percent > 25 { Color::Yellow } else { Color::Red };
        let hp_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(format!(" HP: {}/{} ", combat.enemy.current_hp, combat.enemy.max_hp)))
            .gauge_style(Style::default().fg(hp_color))
            .percent(hp_percent.min(100));
        f.render_widget(hp_gauge, chunks[1]);

        // Typing area - improved for sentences
        let word_display = if combat.phase == CombatPhase::PlayerTurn {
            let typed = &combat.typed_input;
            let target = &combat.current_word;
            let mut spans = Vec::new();
            
            for (i, target_char) in target.chars().enumerate() {
                if i < typed.len() {
                    let typed_char = typed.chars().nth(i).unwrap();
                    if typed_char == target_char {
                        spans.push(Span::styled(
                            target_char.to_string(),
                            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                        ));
                    } else {
                        spans.push(Span::styled(
                            target_char.to_string(),
                            Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED)
                        ));
                    }
                } else if i == typed.len() {
                    // Cursor position - highlight next char
                    spans.push(Span::styled(
                        target_char.to_string(),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                    ));
                } else {
                    spans.push(Span::styled(
                        target_char.to_string(),
                        Style::default().fg(Color::DarkGray)
                    ));
                }
            }
            
            Line::from(spans)
        } else {
            Line::from(format!("{}", combat.current_word))
        };

        // Determine if it's a sentence (longer content)
        let is_sentence = combat.current_word.len() > 30;
        let title_text = if is_sentence {
            format!(" Type the sentence! Combo: {} | Time: {:.1}s | {}/{} chars ", 
                combat.combo, combat.time_remaining, 
                combat.typed_input.len(), combat.current_word.len())
        } else {
            format!(" Type the word! Combo: {} | Time: {:.1}s ", combat.combo, combat.time_remaining)
        };

        let typing_block = Paragraph::new(word_display)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title_text));
        f.render_widget(typing_block, chunks[2]);

        // Player HP
        if let Some(player) = &state.player {
            let player_hp = ((player.hp as f64 / player.max_hp as f64) * 100.0) as u16;
            let player_gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(format!(" Your HP: {}/{} ", player.hp, player.max_hp)))
                .gauge_style(Style::default().fg(Color::Green))
                .percent(player_hp.min(100));
            f.render_widget(player_gauge, chunks[3]);
        }

        // Battle log
        let log_items: Vec<ListItem> = combat.battle_log
            .iter()
            .rev()
            .take(5)
            .map(|msg| ListItem::new(msg.as_str()))
            .collect();
        let log = List::new(log_items)
            .block(Block::default().borders(Borders::ALL).title(" Battle Log "));
        f.render_widget(log, chunks[4]);

        // Help
        let help = Paragraph::new("[Esc] Flee")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[5]);
    }
}

fn render_shop(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    let gold = state.player.as_ref().map(|p| p.gold).unwrap_or(0);
    let header = Paragraph::new(format!("Welcome to the Keyboard Emporium!\n\nYour Gold: {}", gold))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = state.shop_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == state.menu_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(Color::White)
            };
            let text = format!("{} {} - {}g\n  {}", 
                item.rarity.symbol(),
                item.name, 
                item.price,
                item.description
            );
            ListItem::new(text).style(style)
        })
        .collect();

    let items_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Items for Sale "));
    f.render_widget(items_list, chunks[1]);

    let help = Paragraph::new("↑/↓ Select | Enter: Buy | Esc: Leave")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn render_rest(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(f.area());

    let campfire = r#"
        (  .      )
       )           (              )
             .  '   .   '  .  '  .
    (    , )       (.   )  (   ',    )
     .' ) ( . )    ,  ( ,     )   ( .
  ). , ( .   (  ) ( , ')  .' (  ,    )
 (_,_._._._._._._._._._._._._._._._._._)
"#;
    let fire = Paragraph::new(campfire)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" Campfire "));
    f.render_widget(fire, chunks[0]);

    let options = vec![
        "[1] Rest (Restore 30% HP)",
        "[2] Train (Gain some XP)",
        "[3] Meditate (Restore 50% MP)",
    ];
    let options_items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let style = if i == state.menu_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*opt).style(style)
        })
        .collect();
    let rest_list = List::new(options_items)
        .block(Block::default().borders(Borders::ALL).title(" Rest Actions "));
    f.render_widget(rest_list, chunks[1]);

    let help = Paragraph::new("↑/↓ Select | Enter: Confirm | Esc: Leave")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn render_event(f: &mut Frame, state: &GameState) {
    if let Some(event) = &state.current_event {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Length(5),
                Constraint::Min(6),
                Constraint::Length(2),
            ])
            .split(f.area());

        let title = Paragraph::new(&*event.name)
            .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let art = Paragraph::new(&*event.ascii_art)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(art, chunks[1]);

        let desc = Paragraph::new(&*event.description)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(desc, chunks[2]);

        let choices: Vec<ListItem> = event.choices
            .iter()
            .enumerate()
            .map(|(i, choice)| {
                let style = if i == state.menu_index {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::REVERSED)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("[{}] {}", i + 1, choice.text)).style(style)
            })
            .collect();
        let choices_list = List::new(choices)
            .block(Block::default().borders(Borders::ALL).title(" Choices "));
        f.render_widget(choices_list, chunks[3]);

        let help = Paragraph::new("↑/↓ or 1-3: Select | Enter: Confirm")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[4]);
    }
}

fn render_inventory(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("Inventory")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if let Some(player) = &state.player {
        let items: Vec<ListItem> = player.inventory
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == state.menu_index {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                let text = format!("{} {} - {}", item.rarity.symbol(), item.name, item.description);
                ListItem::new(text).style(style)
            })
            .collect();

        if items.is_empty() {
            let empty = Paragraph::new("Your inventory is empty...")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(empty, chunks[1]);
        } else {
            let inv_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Items "));
            f.render_widget(inv_list, chunks[1]);
        }
    }

    let help = Paragraph::new("↑/↓: Select | Enter: Use | Esc: Back")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn render_stats(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(15),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("Character Stats")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    if let Some(player) = &state.player {
        let stats_text = format!(
            r#"
  Name: {}
  Class: {}
  Level: {}
  
  HP: {}/{}
  MP: {}/{}
  
  Strength: {}
  Intellect: {}
  Vitality: {}
  Dexterity: {}
  Luck: {}
  
  Gold: {}
  XP: {}/{}
  
  Session Stats:
  - Enemies Defeated: {}
  - Words Typed: {}
  - Best WPM: {:.1}
"#,
            player.name, player.class.name(), player.level,
            player.hp, player.max_hp,
            player.mp, player.max_mp,
            player.stats.strength, player.stats.intellect,
            player.stats.vitality, player.stats.dexterity,
            player.stats.luck,
            player.gold, player.experience, player.experience_to_next_level(),
            state.total_enemies_defeated, state.total_words_typed, state.best_wpm
        );
        
        let stats = Paragraph::new(stats_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(stats, chunks[1]);
    }

    let help = Paragraph::new("Press any key to return")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn render_game_over(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(f.area());

    let game_over_art = r#"
  ██████╗  █████╗ ███╗   ███╗███████╗
 ██╔════╝ ██╔══██╗████╗ ████║██╔════╝
 ██║  ███╗███████║██╔████╔██║█████╗  
 ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  
 ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗
  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝
  ██████╗ ██╗   ██╗███████╗██████╗ 
 ██╔═══██╗██║   ██║██╔════╝██╔══██╗
 ██║   ██║██║   ██║█████╗  ██████╔╝
 ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗
 ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║
  ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝
"#;

    let title = Paragraph::new(game_over_art)
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let stats = if let Some(player) = &state.player {
        format!(
            "You reached Floor {} as a Level {} {}\n\nEnemies defeated: {}\nWords typed: {}\nBest WPM: {:.1}",
            state.get_current_floor(),
            player.level,
            player.class.name(),
            state.total_enemies_defeated,
            state.total_words_typed,
            state.best_wpm
        )
    } else {
        "Your journey has ended...".to_string()
    };

    let stats_widget = Paragraph::new(stats)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(stats_widget, chunks[1]);

    let help = Paragraph::new("[R] Try Again | [Q] Quit")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn render_victory(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(f.area());

    let victory_art = r#"
 ██╗   ██╗██╗ ██████╗████████╗ ██████╗ ██████╗ ██╗   ██╗██╗
 ██║   ██║██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗╚██╗ ██╔╝██║
 ██║   ██║██║██║        ██║   ██║   ██║██████╔╝ ╚████╔╝ ██║
 ╚██╗ ██╔╝██║██║        ██║   ██║   ██║██╔══██╗  ╚██╔╝  ╚═╝
  ╚████╔╝ ██║╚██████╗   ██║   ╚██████╔╝██║  ██║   ██║   ██╗
   ╚═══╝  ╚═╝ ╚═════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝

          ★ ★ ★ TYPING MASTER ★ ★ ★
"#;

    let title = Paragraph::new(victory_art)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let stats = if let Some(player) = &state.player {
        format!(
            "Congratulations, {}!\n\nYou conquered all 10 floors as a Level {} {}!\n\nEnemies defeated: {}\nWords typed: {}\nBest WPM: {:.1}\n\nYou are a true Typing Champion!",
            player.name,
            player.level,
            player.class.name(),
            state.total_enemies_defeated,
            state.total_words_typed,
            state.best_wpm
        )
    } else {
        "You have conquered the dungeon!".to_string()
    };

    let stats_widget = Paragraph::new(stats)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(stats_widget, chunks[1]);

    let help = Paragraph::new("[N] New Game+ | [Q] Quit")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}
