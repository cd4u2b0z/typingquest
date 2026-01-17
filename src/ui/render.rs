//! Terminal UI rendering using ratatui

use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap, Clear, Tabs},
    Frame,
};
use crate::game::state::{GameState, Scene};
use crate::game::combat::CombatPhase;
use crate::game::help_system::{HelpSystem, HelpTab, TipPriority};

pub fn render(f: &mut Frame, state: &GameState) {
    // Render the main scene
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
    
    // Render help overlay on top if visible
    if state.help_system.visible {
        render_help_overlay(f, &state.help_system, state);
    }
    
    // Always render bottom bar with hint or help reminder
    render_bottom_bar(f, state);
}

/// Render the help overlay as a centered popup
fn render_help_overlay(f: &mut Frame, help: &HelpSystem, state: &GameState) {
    let area = f.area();
    
    // Center popup (80% width, 70% height)
    let popup_width = (area.width as f32 * 0.8) as u16;
    let popup_height = (area.height as f32 * 0.7) as u16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    
    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);
    
    // Clear the area behind the popup
    f.render_widget(Clear, popup_area);
    
    // Main help block
    let help_block = Block::default()
        .title(" 󰋗 HELP ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));
    
    f.render_widget(help_block.clone(), popup_area);
    
    // Inner area for content
    let inner = help_block.inner(popup_area);
    
    // Split into tabs + content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tabs
            Constraint::Min(1),     // Content
            Constraint::Length(2),  // Footer
        ])
        .split(inner);
    
    // Render tabs
    let tab_titles: Vec<Line> = HelpTab::all()
        .iter()
        .map(|t| Line::from(format!(" {} ", t.label())))
        .collect();
    
    let tabs = Tabs::new(tab_titles)
        .select(help.active_tab.index())
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .divider("│");
    
    f.render_widget(tabs, chunks[0]);
    
    // Render content based on active tab
    match help.active_tab {
        HelpTab::Contextual => render_help_contextual(f, help, chunks[1]),
        HelpTab::Keybindings => render_help_keybindings(f, help, chunks[1]),
        HelpTab::Objectives => render_help_objectives(f, help, state, chunks[1]),
        HelpTab::Mechanics => render_help_mechanics(f, help, chunks[1]),
    }
    
    // Footer with navigation hints
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" [1-4] ", Style::default().fg(Color::Yellow)),
        Span::raw("Switch tabs  "),
        Span::styled("[Tab] ", Style::default().fg(Color::Yellow)),
        Span::raw("Next tab  "),
        Span::styled("[j/k] ", Style::default().fg(Color::Yellow)),
        Span::raw("Scroll  "),
        Span::styled("[Esc/?] ", Style::default().fg(Color::Yellow)),
        Span::raw("Close"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::DarkGray));
    
    f.render_widget(footer, chunks[2]);
}

/// Render contextual tips tab
fn render_help_contextual(f: &mut Frame, help: &HelpSystem, area: Rect) {
    let tips = help.get_contextual_tips();
    
    let context_name = match help.context {
        crate::game::help_system::HelpContext::Combat => "Combat",
        crate::game::help_system::HelpContext::Exploration => "Exploration",
        crate::game::help_system::HelpContext::Shop => "Shop",
        crate::game::help_system::HelpContext::Rest => "Rest Site",
        crate::game::help_system::HelpContext::Event => "Event",
        crate::game::help_system::HelpContext::Inventory => "Inventory",
        crate::game::help_system::HelpContext::ClassSelect => "Class Selection",
        crate::game::help_system::HelpContext::Title => "Main Menu",
        _ => "Game",
    };
    
    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled(
                format!("─── Current Context: {} ───", context_name),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ];
    
    for tip in tips.iter().skip(help.scroll_offset) {
        let priority_color = match tip.priority {
            TipPriority::Essential => Color::Green,
            TipPriority::Important => Color::Yellow,
            TipPriority::Advanced => Color::Magenta,
            TipPriority::Secret => Color::Red,
        };
        
        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", tip.icon), Style::default().fg(priority_color)),
            Span::styled(tip.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("     "),
            Span::styled(tip.description, Style::default().fg(Color::Gray)),
        ]));
        lines.push(Line::from(""));
    }
    
    let content = Paragraph::new(lines)
        .block(Block::default())
        .wrap(Wrap { trim: true });
    
    f.render_widget(content, area);
}

/// Render keybindings tab
fn render_help_keybindings(f: &mut Frame, help: &HelpSystem, area: Rect) {
    let bindings = help.get_keybindings(false);
    
    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("─── Keybindings ───", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];
    
    // Group by global vs context-specific
    lines.push(Line::from(Span::styled("  Global:", Style::default().fg(Color::Yellow))));
    for binding in bindings.iter().filter(|b| b.context.is_none()).skip(help.scroll_offset) {
        lines.push(Line::from(vec![
            Span::styled(format!("    {:12}", binding.key), Style::default().fg(Color::Green)),
            Span::styled(binding.action, Style::default().fg(Color::White)),
        ]));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("  Context-Specific:", Style::default().fg(Color::Yellow))));
    for binding in bindings.iter().filter(|b| b.context.is_some()) {
        lines.push(Line::from(vec![
            Span::styled(format!("    {:12}", binding.key), Style::default().fg(Color::Magenta)),
            Span::styled(binding.action, Style::default().fg(Color::White)),
        ]));
    }
    
    let content = Paragraph::new(lines)
        .block(Block::default())
        .wrap(Wrap { trim: true });
    
    f.render_widget(content, area);
}

/// Render objectives tab
fn render_help_objectives(f: &mut Frame, help: &HelpSystem, state: &GameState, area: Rect) {
    let floor = state.dungeon.as_ref().map(|d| d.current_floor).unwrap_or(1);
    let enemies = state.total_enemies_defeated;
    let has_boss = false; // Could track this in dungeon state
    
    let objectives = help.get_objectives(floor, enemies, has_boss);
    
    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("─── Current Objectives ───", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];
    
    for objective in objectives {
        lines.push(Line::from(vec![
            Span::styled(objective.clone(), Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(""));
    }
    
    // Add mystery progress hint
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("─── Mystery ───", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  󰛓 ", Style::default().fg(Color::Magenta)),
        Span::styled("\"The Threshold holds secrets yet unrevealed...\"", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
    ]));
    
    let content = Paragraph::new(lines)
        .block(Block::default())
        .wrap(Wrap { trim: true });
    
    f.render_widget(content, area);
}

/// Render mechanics tab
fn render_help_mechanics(f: &mut Frame, help: &HelpSystem, area: Rect) {
    let mechanics = help.get_mechanics();
    
    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("─── Game Mechanics ───", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];
    
    for (i, (title, subtitle, details)) in mechanics.iter().enumerate().skip(help.scroll_offset) {
        if i > 0 {
            lines.push(Line::from(""));
        }
        
        lines.push(Line::from(vec![
            Span::styled(*title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(format!("  {}", subtitle), Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)),
        ]));
        
        for detail in details {
            lines.push(Line::from(vec![
                Span::styled(format!("    {}", detail), Style::default().fg(Color::White)),
            ]));
        }
    }
    
    let content = Paragraph::new(lines)
        .block(Block::default())
        .wrap(Wrap { trim: true });
    
    f.render_widget(content, area);
}

/// Render bottom bar with hints and help reminder
fn render_bottom_bar(f: &mut Frame, state: &GameState) {
    let area = f.area();
    
    // Bottom bar (1 line at the very bottom)
    let bar_area = Rect::new(0, area.height.saturating_sub(1), area.width, 1);
    
    // Check for active hint first
    let content = if let Some((icon, message)) = state.hint_manager.current_message() {
        Line::from(vec![
            Span::styled(format!(" {} ", icon), Style::default().fg(Color::Yellow)),
            Span::styled(message, Style::default().fg(Color::White)),
        ])
    } else {
        // Default help reminder
        Line::from(vec![
            Span::styled(
                format!(" {} ", state.help_system.get_persistent_hint()),
                Style::default().fg(Color::DarkGray),
            ),
        ])
    };
    
    let bar = Paragraph::new(content)
        .style(Style::default().bg(Color::Rgb(30, 30, 30)));
    
    f.render_widget(bar, bar_area);
}

fn render_title(f: &mut Frame, state: &GameState) {
    let area = f.area();
    
    // Reserve bottom line for key hints
    let main_area = Rect::new(area.x, area.y, area.width, area.height.saturating_sub(2));
    let hint_area = Rect::new(area.x, area.height.saturating_sub(2), area.width, 2);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(12),
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(main_area);

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
    let subtitle = Paragraph::new("A Roguelike Typing Adventure by Dr. Baklava")
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
    
    // Key hints at bottom
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" [j/k] ", Style::default().fg(Color::Yellow)),
        Span::raw("Navigate  "),
        Span::styled("[Enter] ", Style::default().fg(Color::Yellow)),
        Span::raw("Select  "),
        Span::styled("[h] ", Style::default().fg(Color::Cyan)),
        Span::raw("Help  "),
        Span::styled("[q] ", Style::default().fg(Color::Red)),
        Span::raw("Quit"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(Color::Rgb(30, 30, 30)));
    f.render_widget(hints, hint_area);
}

fn render_class_select(f: &mut Frame, state: &GameState) {
    let area = f.area();
    let main_area = Rect::new(area.x, area.y, area.width, area.height.saturating_sub(2));
    let hint_area = Rect::new(area.x, area.height.saturating_sub(2), area.width, 2);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(main_area);

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

    let tip = Paragraph::new("Each class has unique abilities and playstyles")
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center);
    f.render_widget(tip, chunks[2]);
    
    // Key hints at bottom
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" [j/k] ", Style::default().fg(Color::Yellow)),
        Span::raw("Navigate  "),
        Span::styled("[Enter] ", Style::default().fg(Color::Yellow)),
        Span::raw("Select  "),
        Span::styled("[Esc] ", Style::default().fg(Color::Yellow)),
        Span::raw("Back  "),
        Span::styled("[h] ", Style::default().fg(Color::Cyan)),
        Span::raw("Help"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(Color::Rgb(30, 30, 30)));
    f.render_widget(hints, hint_area);
}

fn render_dungeon(f: &mut Frame, state: &GameState) {
    let area = f.area();
    let main_area = Rect::new(area.x, area.y, area.width, area.height.saturating_sub(2));
    let hint_area = Rect::new(area.x, area.height.saturating_sub(2), area.width, 2);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(main_area);

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

    // Message log
    let messages: Vec<Line> = state.message_log.iter()
        .rev()
        .take(2)
        .map(|m| Line::from(Span::styled(m.clone(), Style::default().fg(Color::DarkGray))))
        .collect();
    let log = Paragraph::new(messages)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" Log "));
    f.render_widget(log, chunks[3]);

    // Key hints at bottom
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" [e] ", Style::default().fg(Color::Yellow)),
        Span::raw("Explore  "),
        Span::styled("[i] ", Style::default().fg(Color::Yellow)),
        Span::raw("Inventory  "),
        Span::styled("[s] ", Style::default().fg(Color::Yellow)),
        Span::raw("Stats  "),
        Span::styled("[h] ", Style::default().fg(Color::Cyan)),
        Span::raw("Help  "),
        Span::styled("[q] ", Style::default().fg(Color::Red)),
        Span::raw("Quit"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(Color::Rgb(30, 30, 30)));
    f.render_widget(hints, hint_area);
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

        // Help - key hints for combat
        let help = Paragraph::new(Line::from(vec![
            Span::styled(" [a-z] ", Style::default().fg(Color::Yellow)),
            Span::raw("Type  "),
            Span::styled("[Backspace] ", Style::default().fg(Color::Yellow)),
            Span::raw("Fix  "),
            Span::styled("[Esc] ", Style::default().fg(Color::Red)),
            Span::raw("Flee  "),
            Span::styled("[h] ", Style::default().fg(Color::Cyan)),
            Span::raw("Help"),
        ]))
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Rgb(30, 30, 30)));
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
            "Congratulations, {}!\n\nYou conquered all 10 floors as a Level {} {}!\n\nEnemies defeated: {}\nWords typed: {}\nBest WPM: {:.1}\n\nYou are a true Typing Champion!\n\n󰩛 Dr. Baklava salutes you 󰩛",
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
