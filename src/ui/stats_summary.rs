//! Stats Summary UI - displays combat and run statistics
//!
//! Provides rich visual summaries after:
//! - Each battle (combat summary)  
//! - End of game (run summary, win or lose)

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Gauge, Clear},
    layout::{Layout, Constraint, Direction, Alignment},
    style::{Style, Color, Modifier},
    text::{Line, Span},
};
use crate::game::combat::CombatResult;
use crate::ui::theme::{Palette, Styles};

/// Stats for displaying after a battle
#[derive(Debug, Clone, Default)]
pub struct BattleSummary {
    pub enemy_name: String,
    pub was_boss: bool,
    pub victory: bool,
    pub xp_gained: i32,
    pub gold_gained: i32,
    pub damage_dealt: i32,
    pub damage_taken: i32,
    pub turns_taken: i32,
    pub words_completed: i32,
    pub max_combo: i32,
    pub accuracy: f32,
    pub avg_wpm: f32,
    pub peak_wpm: f32,
    pub perfect_words: i32,
    pub time_elapsed: f32,
}

impl BattleSummary {
    pub fn from_combat_result(result: &CombatResult, enemy_name: &str, was_boss: bool, damage_dealt: i32, damage_taken: i32, time_elapsed: f32) -> Self {
        Self {
            enemy_name: enemy_name.to_string(),
            was_boss,
            victory: result.victory,
            xp_gained: result.xp_gained,
            gold_gained: result.gold_gained,
            damage_dealt,
            damage_taken,
            turns_taken: result.turns_taken,
            words_completed: 0, // TODO: pass from combat
            max_combo: result.max_combo,
            accuracy: result.accuracy,
            avg_wpm: result.avg_wpm,
            peak_wpm: 0.0,
            perfect_words: 0,
            time_elapsed,
        }
    }
}

/// Stats for displaying at end of game
#[derive(Debug, Clone, Default)]
pub struct RunSummary {
    pub player_name: String,
    pub player_class: String,
    pub player_level: i32,
    pub victory: bool,
    pub floors_reached: i32,
    pub enemies_defeated: i32,
    pub bosses_defeated: i32,
    pub total_damage_dealt: i64,
    pub total_damage_taken: i64,
    pub total_words_typed: i32,
    pub total_perfect_words: i32,
    pub overall_accuracy: f32,
    pub best_wpm: f32,
    pub average_wpm: f32,
    pub best_combo: i32,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub items_found: i32,
    pub spells_cast: i32,
    pub time_played_seconds: u64,
    pub ink_earned: u64,
    pub death_cause: Option<String>,
}

/// Render the battle summary overlay after a fight
pub fn render_battle_summary(f: &mut Frame, summary: &BattleSummary) {
    let area = f.area();
    
    // Center the popup
    let popup_width = 50.min(area.width.saturating_sub(4));
    let popup_height = 20.min(area.height.saturating_sub(4));
    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);
    
    // Clear background
    f.render_widget(Clear, popup_area);
    
    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // Stats
            Constraint::Length(2),  // Controls
        ])
        .split(popup_area);
    
    // Title
    let title_text = if summary.victory {
        if summary.was_boss {
            format!("🏆 BOSS DEFEATED: {} 🏆", summary.enemy_name)
        } else {
            format!("⚔️ Victory: {} defeated!", summary.enemy_name)
        }
    } else {
        "💀 Defeated...".to_string()
    };
    
    let title_style = if summary.victory {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    };
    
    let title = Paragraph::new(title_text)
        .style(title_style)
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(if summary.victory { Style::default().fg(Color::Green) } else { Style::default().fg(Color::Red) })
            .title(if summary.was_boss { " 👑 Boss Battle Complete " } else { " ⚔️ Battle Complete " }));
    f.render_widget(title, chunks[0]);
    
    // Stats
    let stats_lines = build_battle_stats_lines(summary);
    let stats = Paragraph::new(stats_lines)
        .alignment(Alignment::Left)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" 📊 Battle Statistics "));
    f.render_widget(stats, chunks[1]);
    
    // Controls
    let controls = Paragraph::new(Line::from(vec![
        Span::styled("[Enter]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Continue"),
    ]))
        .alignment(Alignment::Center);
    f.render_widget(controls, chunks[2]);
}

fn build_battle_stats_lines(summary: &BattleSummary) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    // Rewards
    if summary.victory {
        lines.push(Line::from(vec![
            Span::styled("  Rewards: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(format!("+{} XP", summary.xp_gained), Style::default().fg(Color::Cyan)),
            Span::raw("  "),
            Span::styled(format!("+{} Gold", summary.gold_gained), Style::default().fg(Color::Yellow)),
        ]));
        lines.push(Line::raw(""));
    }
    
    // Combat Performance
    lines.push(Line::from(Span::styled("  ═══ Combat Performance ═══", Style::default().fg(Color::Magenta))));
    
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("󰓥 Damage Dealt: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", summary.damage_dealt), Style::default().fg(Color::Green)),
    ]));
    
    if summary.damage_taken > 0 {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("💥 Damage Taken: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.damage_taken), Style::default().fg(Color::Red)),
        ]));
    } else {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("💥 Damage Taken: ", Style::default().fg(Color::Gray)),
            Span::styled("0 (Flawless!)", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]));
    }
    
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("⏱️ Turns: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}", summary.turns_taken), Style::default().fg(Color::White)),
    ]));
    
    lines.push(Line::raw(""));
    
    // Typing Performance
    lines.push(Line::from(Span::styled("  ═══ Typing Performance ═══", Style::default().fg(Color::Cyan))));
    
    // WPM with color coding
    let wpm_color = if summary.avg_wpm >= 80.0 {
        Color::Green
    } else if summary.avg_wpm >= 50.0 {
        Color::Yellow
    } else {
        Color::White
    };
    
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("󰓅 Average WPM: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1}", summary.avg_wpm), Style::default().fg(wpm_color)),
        if summary.peak_wpm > summary.avg_wpm + 10.0 {
            Span::styled(format!(" (Peak: {:.1})", summary.peak_wpm), Style::default().fg(Color::DarkGray))
        } else {
            Span::raw("")
        },
    ]));
    
    // Accuracy with color coding
    let acc_color = if summary.accuracy >= 0.95 {
        Color::Green
    } else if summary.accuracy >= 0.80 {
        Color::Yellow
    } else {
        Color::Red
    };
    
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("󰇄 Accuracy: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{:.1}%", summary.accuracy * 100.0), Style::default().fg(acc_color)),
    ]));
    
    // Combo
    let combo_color = if summary.max_combo >= 10 {
        Color::Magenta
    } else if summary.max_combo >= 5 {
        Color::Yellow
    } else {
        Color::White
    };
    
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("🔥 Best Combo: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{}x", summary.max_combo), Style::default().fg(combo_color)),
        if summary.max_combo >= 10 {
            Span::styled(" UNSTOPPABLE!", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        } else if summary.max_combo >= 5 {
            Span::styled(" Nice!", Style::default().fg(Color::Yellow))
        } else {
            Span::raw("")
        },
    ]));
    
    if summary.perfect_words > 0 {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("✨ Perfect Words: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.perfect_words), Style::default().fg(Color::Green)),
        ]));
    }
    
    lines
}

/// Render enhanced run summary at game over or victory
pub fn render_run_summary(f: &mut Frame, summary: &RunSummary, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(8),     // Stats
            Constraint::Length(3),  // Ink earned
        ])
        .split(area);
    
    // Header
    let header_text = if summary.victory {
        format!("🏆 {} the {} conquered the dungeon! 🏆", summary.player_name, summary.player_class)
    } else {
        format!("💀 {} the {} fell on Floor {} 💀", summary.player_name, summary.player_class, summary.floors_reached)
    };
    
    let header = Paragraph::new(header_text)
        .style(if summary.victory {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        })
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header, chunks[0]);
    
    // Stats in columns
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);
    
    // Left column: Combat stats
    let combat_stats = build_combat_stats(summary);
    let combat_widget = Paragraph::new(combat_stats)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title(" ⚔️ Combat "));
    f.render_widget(combat_widget, stats_chunks[0]);
    
    // Right column: Typing stats
    let typing_stats = build_typing_stats(summary);
    let typing_widget = Paragraph::new(typing_stats)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" ⌨️ Typing "));
    f.render_widget(typing_widget, stats_chunks[1]);
    
    // Ink earned
    let ink_text = format!(
        "󰙤 Ink Earned: {} | Total Ink: {} | Time: {}",
        summary.ink_earned,
        summary.ink_earned, // This would need total ink from meta progress
        format_time(summary.time_played_seconds)
    );
    let ink_widget = Paragraph::new(ink_text)
        .style(Style::default().fg(Color::Magenta))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(ink_widget, chunks[2]);
}

fn build_combat_stats(summary: &RunSummary) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled(" 󰘛 Floors: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.floors_reached), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(" 󰓥 Enemies: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.enemies_defeated), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled(" 👑 Bosses: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.bosses_defeated), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled(" 💥 Damage Dealt: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.total_damage_dealt), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled(" 🛡️ Damage Taken: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.total_damage_taken), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::styled(" 💰 Gold Earned: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.gold_earned), Style::default().fg(Color::Yellow)),
        ]),
    ]
}

fn build_typing_stats(summary: &RunSummary) -> Vec<Line<'static>> {
    let wpm_color = if summary.best_wpm >= 80.0 { Color::Green } 
        else if summary.best_wpm >= 50.0 { Color::Yellow } 
        else { Color::White };
    
    let acc_color = if summary.overall_accuracy >= 0.95 { Color::Green }
        else if summary.overall_accuracy >= 0.80 { Color::Yellow }
        else { Color::Red };
    
    vec![
        Line::from(vec![
            Span::styled(" 󰌌 Words Typed: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.total_words_typed), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(" ✨ Perfect Words: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", summary.total_perfect_words), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled(" 󰓅 Best WPM: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1}", summary.best_wpm), Style::default().fg(wpm_color)),
        ]),
        Line::from(vec![
            Span::styled(" 󰓅 Avg WPM: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1}", summary.average_wpm), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(" 󰇄 Accuracy: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.1}%", summary.overall_accuracy * 100.0), Style::default().fg(acc_color)),
        ]),
        Line::from(vec![
            Span::styled(" 🔥 Best Combo: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}x", summary.best_combo), Style::default().fg(Color::Magenta)),
        ]),
    ]
}

fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
