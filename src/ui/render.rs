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
use crate::ui::theme::{Palette, Icons, Styles, hp_color, combo_color, wpm_color, accuracy_color, zone_color};
use crate::ui::lore_render::{render_lore_discovery, render_milestone};

pub fn render(f: &mut Frame, state: &GameState) {
    // Render the main scene
    match state.scene {
        Scene::Title => render_title(f, state),
        Scene::ClassSelect => render_class_select(f, state),
        Scene::Dungeon => render_dungeon(f, state),
        Scene::Combat => crate::ui::combat_render::render_combat_enhanced(f, state),
        Scene::Shop => render_shop(f, state),
        Scene::Rest => render_rest(f, state),
        Scene::Event => render_event(f, state),
        Scene::Inventory => render_inventory(f, state),
        Scene::Stats => render_stats(f, state),
        Scene::GameOver => render_game_over(f, state),
        Scene::Victory => render_victory(f, state),
        Scene::Tutorial => render_tutorial(f, state),
        Scene::Lore => render_lore_discovery(f, state),
        Scene::Milestone => render_milestone(f, state),
        Scene::Upgrades => render_upgrades(f, state),
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
        .style(Style::default().fg(Palette::TEXT))
        .highlight_style(Style::default().fg(Palette::WARNING).add_modifier(Modifier::BOLD))
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
        Span::styled(" [1-4] ", Styles::keybind()),
        Span::raw("Switch tabs  "),
        Span::styled("[Tab] ", Styles::keybind()),
        Span::raw("Next tab  "),
        Span::styled("[j/k] ", Styles::keybind()),
        Span::raw("Scroll  "),
        Span::styled("[Esc/?] ", Styles::keybind()),
        Span::raw("Close"),
    ]))
    .alignment(Alignment::Center)
    .style(Styles::dim());
    
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
            TipPriority::Advanced => Palette::ACCENT,
            TipPriority::Secret => Color::Red,
        };
        
        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", tip.icon), Style::default().fg(priority_color)),
            Span::styled(tip.title, Style::default().fg(Palette::TEXT).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("     "),
            Span::styled(tip.description, Style::default().fg(Palette::TEXT_DIM)),
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
    lines.push(Line::from(Span::styled("  Global:", Styles::keybind())));
    for binding in bindings.iter().filter(|b| b.context.is_none()).skip(help.scroll_offset) {
        lines.push(Line::from(vec![
            Span::styled(format!("    {:12}", binding.key), Style::default().fg(Palette::SUCCESS)),
            Span::styled(binding.action, Style::default().fg(Palette::TEXT)),
        ]));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("  Context-Specific:", Styles::keybind())));
    for binding in bindings.iter().filter(|b| b.context.is_some()) {
        lines.push(Line::from(vec![
            Span::styled(format!("    {:12}", binding.key), Style::default().fg(Palette::ACCENT)),
            Span::styled(binding.action, Style::default().fg(Palette::TEXT)),
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
            Span::styled(objective.clone(), Style::default().fg(Palette::TEXT)),
        ]));
        lines.push(Line::from(""));
    }
    
    // Add mystery progress hint
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("─── Mystery ───", Style::default().fg(Palette::ACCENT).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  󰛓 ", Style::default().fg(Palette::ACCENT)),
        Span::styled("\"The Threshold holds secrets yet unrevealed...\"", Styles::dim().add_modifier(Modifier::ITALIC)),
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
            Span::styled(*title, Style::default().fg(Palette::WARNING).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(format!("  {}", subtitle), Style::default().fg(Palette::TEXT_DIM).add_modifier(Modifier::ITALIC)),
        ]));
        
        for detail in details {
            lines.push(Line::from(vec![
                Span::styled(format!("    {}", detail), Style::default().fg(Palette::TEXT)),
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
            Span::styled(format!(" {} ", icon), Styles::keybind()),
            Span::styled(message, Style::default().fg(Palette::TEXT)),
        ])
    } else {
        // Default help reminder
        Line::from(vec![
            Span::styled(
                format!(" {} ", state.help_system.get_persistent_hint()),
                Styles::dim(),
            ),
        ])
    };
    
    let bar = Paragraph::new(content)
        .style(Style::default().bg(Palette::BG_PANEL));
    
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

    // Enhanced ASCII art title with keyboard icon
    let title_art = r#"
╭──────────────────────────────────────────────────────────────────╮
│  ◈═══════════════════════════════════════════════════════════◈  │
│    ████████╗██╗   ██╗██████╗ ██╗███╗   ██╗ ██████╗              │
│    ╚══██╔══╝╚██╗ ██╔╝██╔══██╗██║████╗  ██║██╔════╝              │
│       ██║    ╚████╔╝ ██████╔╝██║██╔██╗ ██║██║  ███╗             │
│       ██║     ╚██╔╝  ██╔═══╝ ██║██║╚██╗██║██║   ██║             │
│       ██║      ██║   ██║     ██║██║ ╚████║╚██████╔╝             │
│       ╚═╝      ╚═╝   ╚═╝     ╚═╝╚═╝  ╚═══╝ ╚═════╝  QUEST  󰌌   │
│  ◈═══════════════════════════════════════════════════════════◈  │
╰──────────────────────────────────────────────────────────────────╯"#;

    let title = Paragraph::new(title_art)
        .style(Style::default().fg(Palette::PRIMARY))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Subtitle with Dr. Baklava icon
    let subtitle = Paragraph::new(Line::from(vec![
        Span::styled("󰩛 ", Style::default().fg(Palette::ACCENT)),
        Span::styled("A Roguelike Typing Adventure by Dr. Baklava", 
            Style::default().fg(Palette::SECONDARY).add_modifier(Modifier::ITALIC)),
        Span::styled(" 󰩛", Style::default().fg(Palette::ACCENT)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(subtitle, chunks[1]);

    // Enhanced menu with icons
    let menu_items = vec![
        ("󰓥", "New Game", "[N]"),
        ("󰂽", "Tutorial", "[T]"),
        ("󰙤", "Upgrades", "[U]"),
        ("󱪙", "Continue", "[C]"),
        ("󰅖", "Quit", "[Q]"),
    ];
    
    // Show ink if any earned
    let ink_display = if state.meta_progress.current_ink > 0 {
        format!("  󰙤 {} Ink", state.meta_progress.current_ink)
    } else {
        String::new()
    };
    
    let menu: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, (icon, text, key))| {
            let (style, icon_color) = if i == state.menu_index {
                (Style::default().fg(Palette::SECONDARY).add_modifier(Modifier::BOLD | Modifier::REVERSED),
                 Palette::SECONDARY)
            } else {
                (Style::default().fg(Palette::TEXT),
                 Palette::PRIMARY)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", icon), Style::default().fg(icon_color)),
                Span::styled(format!("{} {}", key, text), style),
            ]))
        })
        .collect();

    let menu_title = format!(" 󰍜 Menu{} ", ink_display);
    let menu_widget = List::new(menu)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Palette::BORDER)).title(Span::styled(menu_title, Style::default().fg(Palette::PRIMARY))));
    f.render_widget(menu_widget, chunks[2]);
    
    // Key hints at bottom
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" [j/k] ", Styles::keybind()),
        Span::raw("Navigate  "),
        Span::styled("[Enter] ", Styles::keybind()),
        Span::raw("Select  "),
        Span::styled("[?] ", Style::default().fg(Color::Cyan)),
        Span::raw("Help  "),
        Span::styled("[q] ", Style::default().fg(Palette::DANGER)),
        Span::raw("Quit"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(Palette::BG_PANEL));
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
        .style(Style::default().fg(Palette::WARNING).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
    f.render_widget(title, chunks[0]);

    let classes = vec![
        ("Wordsmith", "Balanced fighter. +10% damage, starts with Heal spell.", Color::White),
        ("Scribe", "High MP, spell specialist. +25% MP, learns spells faster.", Color::Blue),
        ("Spellweaver", "Glass cannon mage. +50% spell damage, -20% HP.", Palette::ACCENT),
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
        .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰓥 Classes ", Style::default().fg(Palette::PRIMARY))));
    f.render_widget(class_list, chunks[1]);

    let tip = Paragraph::new("Each class has unique abilities and playstyles")
        .style(Styles::dim().add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center);
    f.render_widget(tip, chunks[2]);
    
    // Key hints at bottom
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" [j/k] ", Styles::keybind()),
        Span::raw("Navigate  "),
        Span::styled("[Enter] ", Styles::keybind()),
        Span::raw("Select  "),
        Span::styled("[Esc] ", Styles::keybind()),
        Span::raw("Back  "),
        Span::styled("[?] ", Style::default().fg(Color::Cyan)),
        Span::raw("Help"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(Palette::BG_PANEL));
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

    // Header with floor info and zone name
    let floor = state.get_current_floor();
    let zone_name = state.dungeon.as_ref()
        .map(|d| d.zone_name.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    let header = Paragraph::new(format!("Floor {} — {}", floor, zone_name))
        .style(Styles::title())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&zone_name))));
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
            .style(Style::default().fg(Palette::TEXT))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(format!(" {} - {} ", player.name, player.class.name())));
        f.render_widget(stats, chunks[1]);
    }

    // Room display / map
    if let Some(dungeon) = &state.dungeon {
        let room_display = dungeon.get_ascii_map();
        let room = Paragraph::new(room_display)
            .style(Styles::keybind())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰍋 Dungeon Map ", Style::default().fg(Palette::PRIMARY))));
        f.render_widget(room, chunks[2]);
    }

    // Message log
    let messages: Vec<Line> = state.message_log.iter()
        .rev()
        .take(2)
        .map(|m| Line::from(Span::styled(m.clone(), Styles::dim())))
        .collect();
    let log = Paragraph::new(messages)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰎟 Log ", Style::default().fg(Palette::TEXT_DIM))));
    f.render_widget(log, chunks[3]);

    // Key hints at bottom - make EXPLORE very prominent
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" [Enter/e] ", Styles::typed_correct()),
        Span::styled("EXPLORE ", Styles::typed_correct()),
        Span::styled("[i] ", Styles::keybind()),
        Span::raw("Inventory  "),
        Span::styled("[s] ", Styles::keybind()),
        Span::raw("Stats  "),
        Span::styled("[?] ", Style::default().fg(Color::Cyan)),
        Span::raw("Help  "),
        Span::styled("[q] ", Style::default().fg(Palette::DANGER)),
        Span::raw("Quit"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(Palette::BG_PANEL));
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
            .style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown"))))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
        f.render_widget(enemy_widget, chunks[0]);

        // Enemy HP bar
        let hp_percent = ((combat.enemy.current_hp as f64 / combat.enemy.max_hp as f64) * 100.0) as u16;
        let hp_color = if hp_percent > 50 { Palette::SUCCESS } else if hp_percent > 25 { Palette::WARNING } else { Palette::DANGER };
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
                            Styles::typed_correct()
                        ));
                    } else {
                        spans.push(Span::styled(
                            target_char.to_string(),
                            Styles::typed_wrong()
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
                        Styles::dim()
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
                .gauge_style(Style::default().fg(Palette::SUCCESS))
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
            .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰵅 Battle Log ", Style::default().fg(Palette::INFO))));
        f.render_widget(log, chunks[4]);

        // Help - key hints for combat (context-sensitive)
        let help_spans = if combat.spell_mode {
            vec![
                Span::styled(" [1-9] ", Styles::keybind()),
                Span::raw("Cast Spell  "),
                Span::styled("[Tab] ", Style::default().fg(Color::Cyan)),
                Span::raw("Cancel  "),
                Span::styled("[Esc] ", Style::default().fg(Palette::DANGER)),
                Span::raw("Flee"),
            ]
        } else {
            vec![
                Span::styled(" [a-z] ", Styles::keybind()),
                Span::raw("Type  "),
                Span::styled("[Tab] ", Style::default().fg(Color::Magenta)),
                Span::raw("󰊠 Spells  "),
                Span::styled("[Backspace] ", Styles::keybind()),
                Span::raw("Fix  "),
                Span::styled("[Esc] ", Style::default().fg(Palette::DANGER)),
                Span::raw("Flee"),
            ]
        };
        let help = Paragraph::new(Line::from(help_spans))
            .alignment(Alignment::Center)
            .style(Style::default().bg(Palette::BG_PANEL));
        f.render_widget(help, chunks[5]);
        
        // Render typing feel overlay
        render_typing_feel_overlay(f, state, f.area());
        
        // Render visual effects overlay (floating damage, hit flash, etc.)
        render_effects_overlay(f, state, f.area());
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
        .style(Styles::keybind())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = state.shop_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == state.menu_index {
                Styles::keybind().add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(Palette::TEXT)
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
        .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰆼 Items for Sale ", Style::default().fg(Palette::SECONDARY))));
    f.render_widget(items_list, chunks[1]);

    let help = Paragraph::new("↑/↓ Select | Enter: Buy | Esc: Leave")
        .style(Styles::dim())
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
        .style(Styles::keybind())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰈸 Campfire ", Style::default().fg(Palette::WARNING))));
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
                Style::default().fg(Palette::WARNING).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Palette::TEXT)
            };
            ListItem::new(*opt).style(style)
        })
        .collect();
    let rest_list = List::new(options_items)
        .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰣐 Rest Actions ", Style::default().fg(Palette::SUCCESS))));
    f.render_widget(rest_list, chunks[1]);

    let help = Paragraph::new("↑/↓ Select | Enter: Confirm | Esc: Leave")
        .style(Styles::dim())
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
            .style(Style::default().fg(Palette::ACCENT).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
        f.render_widget(title, chunks[0]);

        let art = Paragraph::new(&*event.ascii_art)
            .style(Style::default().fg(Palette::PRIMARY))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
        f.render_widget(art, chunks[1]);

        let desc = Paragraph::new(&*event.description)
            .style(Style::default().fg(Palette::TEXT))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
        f.render_widget(desc, chunks[2]);

        let choices: Vec<ListItem> = event.choices
            .iter()
            .enumerate()
            .map(|(i, choice)| {
                let style = if i == state.menu_index {
                    Styles::keybind().add_modifier(Modifier::BOLD | Modifier::REVERSED)
                } else {
                    Style::default().fg(Palette::TEXT)
                };
                ListItem::new(format!("[{}] {}", i + 1, choice.text)).style(style)
            })
            .collect();
        let choices_list = List::new(choices)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰋗 Choices ", Style::default().fg(Palette::INFO))));
        f.render_widget(choices_list, chunks[3]);

        let help = Paragraph::new("↑/↓ or 1-3: Select | Enter: Confirm")
            .style(Styles::dim())
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
        .style(Style::default().fg(Palette::WARNING).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
    f.render_widget(title, chunks[0]);

    if let Some(player) = &state.player {
        let items: Vec<ListItem> = player.inventory
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == state.menu_index {
                    Style::default().fg(Palette::WARNING).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Palette::TEXT)
                };
                let text = format!("{} {} - {}", item.rarity.symbol(), item.name, item.description);
                ListItem::new(text).style(style)
            })
            .collect();

        if items.is_empty() {
            let empty = Paragraph::new("Your inventory is empty...")
                .style(Styles::dim())
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
            f.render_widget(empty, chunks[1]);
        } else {
            let inv_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󱋣 Items ", Style::default().fg(Palette::SECONDARY))));
            f.render_widget(inv_list, chunks[1]);
        }
    }

    let help = Paragraph::new("↑/↓: Select | Enter: Use | Esc: Back")
        .style(Styles::dim())
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn render_stats(f: &mut Frame, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(8),  // Faction standings
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("Character Stats")
        .style(Style::default().fg(Palette::WARNING).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
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
            .style(Style::default().fg(Palette::TEXT))
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
        f.render_widget(stats, chunks[1]);
    }

    // Faction standings
    let factions = &state.faction_relations;
    let faction_text = format!(
        "󰜃 Faction Standings 󰜃\n\n  󰂡 Scribes: {}  󰬲 Mechanists: {}  󰌪 Naturalists: {}\n  󰬡 Shadow Writers: {}  󰏮 Archivists: {}",
        format_standing(factions.standings.get(&crate::game::narrative::Faction::MagesGuild).copied().unwrap_or(0)),
        format_standing(factions.standings.get(&crate::game::narrative::Faction::TempleOfDawn).copied().unwrap_or(0)),
        format_standing(factions.standings.get(&crate::game::narrative::Faction::RangersOfTheWild).copied().unwrap_or(0)),
        format_standing(factions.standings.get(&crate::game::narrative::Faction::ShadowGuild).copied().unwrap_or(0)),
        format_standing(factions.standings.get(&crate::game::narrative::Faction::MerchantConsortium).copied().unwrap_or(0)),
    );
    let faction_widget = Paragraph::new(faction_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
    f.render_widget(faction_widget, chunks[2]);
    
    let help = Paragraph::new("Press any key to return")
        .style(Styles::dim())
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[3]);
}

/// Format a faction standing as a colored string
fn format_standing(standing: i32) -> String {
    if standing >= 50 { format!("󰄬 {}", standing) }
    else if standing >= 25 { format!("󰆓 {}", standing) }
    else if standing > -25 { format!("{}", standing) }
    else if standing > -50 { format!("󰆗 {}", standing) }
    else { format!("󰀧 {}", standing) }
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
        .style(Style::default().fg(Palette::DANGER))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let stats = if let Some(player) = &state.player {
        format!(
            "󰯈 You reached Floor {} as a Level {} {}\n\n󰓥 Enemies defeated: {}\n󰌌 Words typed: {}\n󰓅 Best WPM: {:.1}\n\n󰙤 Ink Earned: {} (Total: {})\n\n\"The keyboard awaits your return...\"",
            state.get_current_floor(),
            player.level,
            player.class.name(),
            state.total_enemies_defeated,
            state.total_words_typed,
            state.best_wpm,
            state.meta_progress.current_ink,
            state.meta_progress.total_ink
        )
    } else {
        "󰯈 Your journey has ended...".to_string()
    };

    let stats_widget = Paragraph::new(stats)
        .style(Style::default().fg(Palette::TEXT))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
    f.render_widget(stats_widget, chunks[1]);

    let help = Paragraph::new(Line::from(vec![Span::styled("󰓥 ", Style::default().fg(Palette::SUCCESS)), Span::styled("[R] Try Again  ", Styles::keybind()), Span::styled("󰅖 ", Style::default().fg(Palette::DANGER)), Span::styled("[Q] Quit", Style::default().fg(Palette::DANGER))]))
        .style(Styles::keybind())
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
        .style(Styles::keybind())
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let stats = if let Some(player) = &state.player {
        format!(
            "󰔰 Congratulations, {}! 󰔰\n\n󰘛 You conquered all 10 floors as a Level {} {}!\n\n󰓥 Enemies defeated: {}\n󰌌 Words typed: {}\n󰓅 Best WPM: {:.1}\n\n★ ★ ★ You are a true Typing Champion! ★ ★ ★\n\n󰩛 Dr. Baklava salutes you 󰩛",
            player.name,
            player.level,
            player.class.name(),
            state.total_enemies_defeated,
            state.total_words_typed,
            state.best_wpm
        )
    } else {
        "󰔰 You have conquered the dungeon! 󰔰".to_string()
    };

    let stats_widget = Paragraph::new(stats)
        .style(Style::default().fg(Palette::TEXT))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
    f.render_widget(stats_widget, chunks[1]);

    let help = Paragraph::new(Line::from(vec![Span::styled("󰓥 ", Style::default().fg(Palette::SUCCESS)), Span::styled("[N] New Game+  ", Styles::keybind()), Span::styled("󰅖 ", Style::default().fg(Palette::DANGER)), Span::styled("[Q] Quit", Style::default().fg(Palette::DANGER))]))
        .style(Styles::keybind())
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn render_tutorial(f: &mut Frame, state: &GameState) {
    let area = f.area();
    let tutorial = &state.tutorial_state;
    
    // Get current step - return early with placeholder if none
    let step = match tutorial.current_step() {
        Some(s) => s,
        None => {
            let placeholder = Paragraph::new("Tutorial Complete!")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Palette::SUCCESS));
            f.render_widget(placeholder, area);
            return;
        }
    };
    
    // Main layout: title, content area, input area, hints
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),   // Title and phase indicator
            Constraint::Min(8),      // Narrative and instructions
            Constraint::Length(5),   // Typing area
            Constraint::Length(3),   // Progress bar
            Constraint::Length(2),   // Key hints
        ])
        .split(area);
    
    // Phase title with decorative border
    let phase = tutorial.current_phase();
    let phase_icon = match phase {
        crate::game::tutorial::TutorialPhase::Awakening => "󰛨",
        crate::game::tutorial::TutorialPhase::FirstStrike => "󰓥",
        crate::game::tutorial::TutorialPhase::TheCombo => "󱋊",
        crate::game::tutorial::TutorialPhase::Choice => "󰋗",
        crate::game::tutorial::TutorialPhase::Discovery => "󰈈",
        crate::game::tutorial::TutorialPhase::Complete => "󰄬",
    };
    
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            "  Tutorial ",
            Style::default().fg(Palette::WARNING).add_modifier(Modifier::BOLD),
        ));
    
    let title_text = format!("{} {}", phase_icon, phase.title());
    let title = Paragraph::new(title_text)
        .style(Styles::title())
        .alignment(Alignment::Center)
        .block(title_block);
    f.render_widget(title, chunks[0]);
    
    // Narrative and instructions
    let narrative_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Styles::dim())
        .title(Span::styled(
            " Story ",
            Style::default().fg(Palette::ACCENT),
        ));
    
    let mut lines = vec![
        Line::from(Span::styled(
            step.narrative,
            Style::default().fg(Palette::TEXT),
        )),
        Line::from(""),
        Line::from(Span::styled(
            step.hint,
            Style::default().fg(Palette::WARNING).add_modifier(Modifier::ITALIC),
        )),
    ];
    
    // Show combo feedback for TheCombo phase
    if matches!(phase, crate::game::tutorial::TutorialPhase::TheCombo) {
        lines.push(Line::from(""));
        let combo = tutorial.current_combo();
        let combo_text = if combo > 0 {
            format!("󱋊 Combo: {}x", combo)
        } else {
            "󱋊 Build your combo by typing correctly!".to_string()
        };
        lines.push(Line::from(Span::styled(
            combo_text,
            Style::default().fg(Palette::FLOW_TRANSCENDENT).add_modifier(Modifier::BOLD),
        )));
    }
    
    let narrative = Paragraph::new(lines)
        .block(narrative_block)
        .wrap(Wrap { trim: true });
    f.render_widget(narrative, chunks[1]);
    
    // Typing area - show target and current input
    let typing_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Palette::SUCCESS))
        .title(Span::styled(
            " Type this ",
            Style::default().fg(Palette::SUCCESS),
        ));
    
    // Build colored text showing correct/incorrect characters
    let target = &step.target_text;
    let input = &tutorial.typed_input;
    
    let mut spans = Vec::new();
    for (i, target_char) in target.chars().enumerate() {
        if let Some(input_char) = input.chars().nth(i) {
            if input_char == target_char {
                spans.push(Span::styled(
                    target_char.to_string(),
                    Styles::typed_correct(),
                ));
            } else {
                spans.push(Span::styled(
                    target_char.to_string(),
                    Styles::typed_wrong(),
                ));
            }
        } else {
            spans.push(Span::styled(
                target_char.to_string(),
                Styles::dim(),
            ));
        }
    }
    
    let typing_line = Line::from(spans);
    let typing = Paragraph::new(vec![
        typing_line,
        Line::from(""),
        Line::from(Span::styled(
            format!("> {}_", input),
            Style::default().fg(Color::Cyan),
        )),
    ])
    .block(typing_block)
    .alignment(Alignment::Center);
    f.render_widget(typing, chunks[2]);
    
    // Progress bar showing tutorial completion
    let progress_ratio = tutorial.progress_percent() as f64 / 100.0;
    let progress = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(Span::styled(" 󰓎 Progress ", Style::default().fg(Palette::PRIMARY))))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(progress_ratio)
        .label(format!("{}%", tutorial.progress_percent()));
    f.render_widget(progress, chunks[3]);
    
    // Key hints
    let complete_hint = if tutorial.is_step_complete() {
        "[Enter] Continue  "
    } else {
        ""
    };
    let hints = Paragraph::new(format!(
        "{}[Tab] Skip Step  [Esc] Exit Tutorial",
        complete_hint
    ))
    .style(Styles::dim())
    .alignment(Alignment::Center);
    f.render_widget(hints, chunks[4]);
}

/// Render meta-progression upgrades shop
fn render_upgrades(f: &mut Frame, state: &GameState) {
    let area = f.area();
    let main_area = Rect::new(area.x, area.y, area.width, area.height.saturating_sub(2));
    let hint_area = Rect::new(area.x, area.height.saturating_sub(2), area.width, 2);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
        ])
        .split(main_area);
    
    // Header with ink display
    let header_text = vec![
        Line::from(vec![
            Span::styled("󰙤 ", Style::default().fg(Palette::ACCENT)),
            Span::styled("INK SHOP", Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD)),
            Span::styled(" 󰙤", Style::default().fg(Palette::ACCENT)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Available Ink: "),
            Span::styled(format!("{}", state.meta_progress.current_ink), Style::default().fg(Palette::ACCENT).add_modifier(Modifier::BOLD)),
            Span::raw("   Total Earned: "),
            Span::styled(format!("{}", state.meta_progress.total_ink), Style::default().fg(Palette::SECONDARY)),
        ]),
    ];
    
    let header = Paragraph::new(header_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Palette::BORDER)));
    f.render_widget(header, chunks[0]);
    
    // Upgrades list
    let upgrades = state.meta_progress.get_available_upgrades();
    
    let items: Vec<ListItem> = if upgrades.is_empty() {
        vec![ListItem::new(Line::from(vec![
            Span::styled("No upgrades available yet!", Style::default().fg(Palette::SECONDARY).add_modifier(Modifier::ITALIC)),
        ]))]
    } else {
        upgrades.iter().enumerate().map(|(i, upgrade)| {
            let is_selected = i == state.menu_index;
            let can_afford = state.meta_progress.current_ink >= upgrade.cost;
            
            let (style, cost_color) = if is_selected {
                (Style::default().fg(Palette::SECONDARY).add_modifier(Modifier::REVERSED),
                 if can_afford { Palette::SUCCESS } else { Palette::DANGER })
            } else {
                (Style::default().fg(if can_afford { Palette::TEXT } else { Color::DarkGray }),
                 if can_afford { Palette::TEXT } else { Color::DarkGray })
            };
            
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!(" {} ", upgrade.category.icon()), Style::default().fg(Palette::PRIMARY)),
                    Span::styled(&upgrade.name, style.add_modifier(Modifier::BOLD)),
                    Span::raw(" "),
                    Span::styled(format!("[{} Ink]", upgrade.cost), Style::default().fg(cost_color)),
                ]),
                Line::from(vec![
                    Span::raw("   "),
                    Span::styled(&upgrade.description, Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)),
                ]),
            ])
        }).collect()
    };
    
    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Palette::BORDER))
            .title(Span::styled(" 󱃵 Permanent Upgrades ", Style::default().fg(Palette::PRIMARY))));
    f.render_widget(list, chunks[1]);
    
    // Key hints
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" [j/k] ", Styles::keybind()),
        Span::raw("Navigate  "),
        Span::styled("[Enter] ", Styles::keybind()),
        Span::raw("Purchase  "),
        Span::styled("[Esc] ", Style::default().fg(Palette::WARNING)),
        Span::raw("Back to Menu"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(Palette::BG_PANEL));
    f.render_widget(hints, hint_area);
}

/// Render typing feel effects overlay on combat screen
fn render_typing_feel_overlay(f: &mut Frame, state: &GameState, area: Rect) {
    let feel = &state.typing_feel;
    
    // Combo display in top-right corner
    if feel.combo > 0 {
        let combo_width = 20;
        let combo_height = 3;
        let combo_area = Rect::new(
            area.width.saturating_sub(combo_width + 2),
            1,
            combo_width,
            combo_height,
        );
        
        let combo_text = if feel.combo >= 10 {
            format!("󱋊 {} COMBO! 󱋊\nx{:.1} DMG", feel.combo, feel.combo_multiplier)
        } else {
            format!("󱋊 {} Combo\nx{:.1} DMG", feel.combo, feel.combo_multiplier)
        };
        
        let combo_color = if feel.combo >= 20 {
            Palette::ACCENT
        } else if feel.combo >= 10 {
            Color::Red
        } else if feel.combo >= 5 {
            Color::Yellow
        } else {
            Color::Cyan
        };
        
        let combo_widget = Paragraph::new(combo_text)
            .style(Style::default().fg(combo_color).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(zone_color(&state.dungeon.as_ref().map(|d| d.zone_name.as_str()).unwrap_or("Unknown")))));
        f.render_widget(combo_widget, combo_area);
    }
    
    // Flow state indicator
    let flow_desc = feel.flow_description();
    if !flow_desc.is_empty() {
        let flow_width = 25;
        let flow_area = Rect::new(
            2,
            1,
            flow_width,
            1,
        );
        
        let flow_color = match feel.flow_state {
            crate::game::typing_feel::FlowState::Transcendent => Palette::ACCENT,
            crate::game::typing_feel::FlowState::Flowing => Color::Cyan,
            crate::game::typing_feel::FlowState::Building => Color::Yellow,
            crate::game::typing_feel::FlowState::Recovering => Color::Red,
        };
        
        let flow_text = Span::styled(
            format!("󰔟 {}", flow_desc),
            Style::default().fg(flow_color).add_modifier(Modifier::ITALIC),
        );
        let flow_widget = Paragraph::new(flow_text);
        f.render_widget(flow_widget, flow_area);
    }
    
    // WPM display
    if feel.wpm > 0.0 {
        let wpm_width = 15;
        let wpm_area = Rect::new(
            area.width / 2 - wpm_width / 2,
            area.height.saturating_sub(3),
            wpm_width,
            1,
        );
        
        let wpm_color = if feel.wpm >= 80.0 {
            Palette::ACCENT
        } else if feel.wpm >= 60.0 {
            Color::Yellow
        } else if feel.wpm >= 40.0 {
            Color::Cyan
        } else {
            Color::White
        };
        
        let wpm_text = Span::styled(
            format!("󰓅 {:.0} WPM", feel.wpm),
            Style::default().fg(wpm_color).add_modifier(Modifier::BOLD),
        );
        let wpm_widget = Paragraph::new(wpm_text).alignment(Alignment::Center);
        f.render_widget(wpm_widget, wpm_area);
    }
    
    // Accuracy display
    if feel.accuracy > 0.0 {
        let acc_width = 15;
        let acc_area = Rect::new(
            2,
            area.height.saturating_sub(3),
            acc_width,
            1,
        );
        
        let acc_color = if feel.accuracy >= 95.0 {
            Color::Green
        } else if feel.accuracy >= 85.0 {
            Color::Yellow
        } else {
            Color::Red
        };
        
        let acc_text = Span::styled(
            format!("󰄬 {:.0}% ACC", feel.accuracy),
            Style::default().fg(acc_color),
        );
        let acc_widget = Paragraph::new(acc_text);
        f.render_widget(acc_widget, acc_area);
    }
}

/// Render visual effects overlay (floating damage, screen shake, hit flash)
fn render_effects_overlay(f: &mut Frame, state: &GameState, area: Rect) {
    use crate::ui::effects::{TextColor, TextSize};
    
    // Render floating texts
    for text in &state.effects.floating_texts {
        if text.is_expired() {
            continue;
        }
        
        let x = (area.x as f32 + area.width as f32 * text.x) as u16;
        let y = (area.y as f32 + area.height as f32 * text.current_y().max(0.0).min(1.0)) as u16;
        
        if y >= area.y && y < area.y + area.height && x >= area.x && x < area.x + area.width {
            let color = match text.color {
                TextColor::Damage => Color::Red,
                TextColor::Critical => Color::Yellow,
                TextColor::Heal => Color::Green,
                TextColor::Combo => Color::Cyan,
                TextColor::Miss => Color::DarkGray,
                TextColor::Perfect => Color::White,
                TextColor::Bonus => Color::Magenta,
            };
            
            let mut style = Style::default().fg(color);
            match text.size {
                TextSize::Huge | TextSize::Large => {
                    style = style.add_modifier(Modifier::BOLD);
                }
                _ => {}
            }
            
            // Fade effect
            if text.current_opacity() < 0.5 {
                style = style.add_modifier(Modifier::DIM);
            }
            
            let text_len = text.text.len() as u16;
            let text_area = Rect {
                x: x.saturating_sub(text_len / 2).max(area.x),
                y,
                width: text_len.min(area.width),
                height: 1,
            };
            
            let widget = Paragraph::new(text.text.clone())
                .style(style)
                .alignment(Alignment::Center);
            f.render_widget(widget, text_area);
        }
    }
    
    // Hit flash effect - color the border
    if let Some(ref flash) = state.effects.hit_flash {
        if flash.is_active() {
            let flash_color = match flash.color {
                crate::ui::effects::FlashColor::Red => Color::Red,
                crate::ui::effects::FlashColor::White => Color::White,
                crate::ui::effects::FlashColor::Yellow => Color::Yellow,
                crate::ui::effects::FlashColor::Green => Color::Green,
            };
            
            // Render a flash border
            let flash_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(flash_color).add_modifier(Modifier::BOLD));
            f.render_widget(flash_block, area);
        }
    }
    
    // Combo pulse indicator
    if let Some(ref pulse) = state.effects.combo_pulse {
        if pulse.is_active() {
            let pulse_text = format!("🔥 {}x COMBO! 🔥", pulse.combo);
            let pulse_width = pulse_text.len() as u16 + 4;
            let pulse_area = Rect {
                x: area.width / 2 - pulse_width / 2,
                y: 2,
                width: pulse_width,
                height: 1,
            };
            
            let widget = Paragraph::new(pulse_text)
                .style(Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            f.render_widget(widget, pulse_area);
        }
    }
}
