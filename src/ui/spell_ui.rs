//! Spell UI Components - Spell bar, cooldowns, and casting visualization
//!
//! Provides rich spell UI for combat including:
//! - Spell hotbar with cooldown indicators
//! - Active spell casting visualization
//! - Mana bar with spell cost preview
//! - Spell selection overlay

use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, Paragraph, Clear},
    Frame,
};

use crate::game::player::Player;
use crate::game::spells::{Spell, SpellElement, SpellEffect};
use crate::game::combat::CombatState;

/// Spell cooldown tracking for combat
#[derive(Debug, Clone, Default)]
pub struct SpellCooldowns {
    /// Cooldown remaining for each spell slot (0 = ready)
    pub cooldowns: Vec<u32>,
    /// Recently cast spell for animation
    pub last_cast: Option<(usize, std::time::Instant)>,
}

impl SpellCooldowns {
    pub fn new(spell_count: usize) -> Self {
        Self {
            cooldowns: vec![0; spell_count],
            last_cast: None,
        }
    }

    pub fn is_ready(&self, slot: usize) -> bool {
        self.cooldowns.get(slot).map(|&cd| cd == 0).unwrap_or(false)
    }

    pub fn trigger_cooldown(&mut self, slot: usize, turns: u32) {
        if let Some(cd) = self.cooldowns.get_mut(slot) {
            *cd = turns;
        }
        self.last_cast = Some((slot, std::time::Instant::now()));
    }

    pub fn tick_cooldowns(&mut self) {
        for cd in &mut self.cooldowns {
            if *cd > 0 {
                *cd -= 1;
            }
        }
    }
}

/// Render the spell hotbar at the bottom of the combat screen
pub fn render_spell_bar(
    f: &mut Frame,
    player: &Player,
    combat: &CombatState,
    cooldowns: &SpellCooldowns,
    area: Rect,
) {
    let spells = &player.known_spells;
    if spells.is_empty() {
        return;
    }

    // Calculate spell slot widths
    let slot_count = spells.len().min(9);
    let slot_width = (area.width / slot_count as u16).max(10);

    let constraints: Vec<Constraint> = (0..slot_count)
        .map(|_| Constraint::Length(slot_width))
        .collect();

    let spell_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, spell) in spells.iter().take(9).enumerate() {
        let is_selected = combat.selected_spell == Some(i);
        let is_ready = cooldowns.is_ready(i);
        let can_afford = player.mp >= spell.mp_cost;
        let cooldown = cooldowns.cooldowns.get(i).copied().unwrap_or(0);

        render_spell_slot(
            f,
            spell,
            i + 1,
            is_selected,
            is_ready,
            can_afford,
            cooldown,
            spell_chunks[i],
        );
    }
}

fn render_spell_slot(
    f: &mut Frame,
    spell: &Spell,
    slot_num: usize,
    is_selected: bool,
    is_ready: bool,
    can_afford: bool,
    cooldown: u32,
    area: Rect,
) {
    let element_color = element_to_color(&spell.element);
    
    // Determine slot style
    let (border_color, text_style) = if is_selected {
        (Color::White, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
    } else if !is_ready {
        (Color::DarkGray, Style::default().fg(Color::DarkGray))
    } else if !can_afford {
        (Color::Red, Style::default().fg(Color::Red).add_modifier(Modifier::DIM))
    } else {
        (element_color, Style::default().fg(element_color))
    };

    // Slot number and icon
    let icon = element_icon(&spell.element);
    let header = format!("[{}] {}", slot_num, icon);

    // Spell name (truncated if needed)
    let name = if spell.name.len() > area.width as usize - 2 {
        format!("{}…", &spell.name[..area.width as usize - 3])
    } else {
        spell.name.clone()
    };

    // MP cost or cooldown indicator
    let status = if cooldown > 0 {
        format!("CD:{}", cooldown)
    } else {
        format!("{}mp", spell.mp_cost)
    };

    let content = vec![
        Line::from(Span::styled(header, text_style)),
        Line::from(Span::styled(name, text_style)),
        Line::from(Span::styled(status, text_style.fg(if can_afford && is_ready { Color::Cyan } else { Color::DarkGray }))),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(if is_selected {
            Style::default().bg(Color::Rgb(30, 30, 50))
        } else {
            Style::default()
        });

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

/// Render spell selection overlay when in spell mode
pub fn render_spell_selection_overlay(
    f: &mut Frame,
    player: &Player,
    combat: &CombatState,
    cooldowns: &SpellCooldowns,
    area: Rect,
) {
    // Semi-transparent background
    let overlay_area = centered_rect(70, 60, area);
    f.render_widget(Clear, overlay_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .title(Span::styled(" ✨ Spells ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)))
        .style(Style::default().bg(Color::Rgb(20, 10, 30)));

    f.render_widget(block.clone(), overlay_area);

    // Inner area for spell list
    let inner = Block::default().inner(overlay_area);

    let spells = &player.known_spells;
    let lines: Vec<Line> = spells
        .iter()
        .enumerate()
        .take(9)
        .map(|(i, spell)| {
            let is_selected = combat.selected_spell == Some(i);
            let is_ready = cooldowns.is_ready(i);
            let can_afford = player.mp >= spell.mp_cost;
            let cooldown = cooldowns.cooldowns.get(i).copied().unwrap_or(0);

            let icon = element_icon(&spell.element);
            let element_color = element_to_color(&spell.element);

            let prefix = if is_selected { "▶ " } else { "  " };
            let num_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
            let name_style = if !is_ready || !can_afford {
                Style::default().fg(Color::DarkGray)
            } else if is_selected {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(element_color)
            };

            let cost_color = if !can_afford {
                Color::Red
            } else {
                Color::Cyan
            };

            let status = if cooldown > 0 {
                format!(" [CD:{}]", cooldown)
            } else {
                String::new()
            };

            Line::from(vec![
                Span::raw(prefix),
                Span::styled(format!("[{}] ", i + 1), num_style),
                Span::styled(format!("{} ", icon), Style::default().fg(element_color)),
                Span::styled(format!("{:<16}", spell.name), name_style),
                Span::styled(format!(" {:>3}mp", spell.mp_cost), Style::default().fg(cost_color)),
                Span::styled(status, Style::default().fg(Color::DarkGray)),
                Span::styled(format!("  {}", spell.description), Style::default().fg(Color::DarkGray)),
            ])
        })
        .collect();

    let help_line = Line::from(vec![
        Span::styled("[1-9]", Style::default().fg(Color::Yellow)),
        Span::raw(" Select  "),
        Span::styled("[Enter]", Style::default().fg(Color::Green)),
        Span::raw(" Cast  "),
        Span::styled("[Tab/Esc]", Style::default().fg(Color::Red)),
        Span::raw(" Cancel"),
    ]);

    let mut all_lines = lines;
    all_lines.push(Line::from(""));
    all_lines.push(help_line);

    let paragraph = Paragraph::new(all_lines).alignment(Alignment::Left);
    f.render_widget(paragraph, inner);
}

/// Render spell casting progress bar
pub fn render_spell_casting(
    f: &mut Frame,
    spell: &Spell,
    typed: &str,
    time_remaining: f32,
    area: Rect,
) {
    let element_color = element_to_color(&spell.element);
    let icon = element_icon(&spell.element);

    // Incantation display with typed progress
    let incantation = &spell.incantation;
    let typed_len = typed.len().min(incantation.len());
    let remaining = &incantation[typed_len..];

    let incantation_spans = vec![
        Span::styled(icon.to_string(), Style::default().fg(element_color)),
        Span::raw(" "),
        Span::styled(
            &incantation[..typed_len],
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            remaining,
            Style::default().fg(Color::White).add_modifier(Modifier::DIM),
        ),
    ];

    // Time bar
    let time_pct = (time_remaining / spell.cast_time).clamp(0.0, 1.0);
    let time_color = if time_pct > 0.5 {
        Color::Green
    } else if time_pct > 0.25 {
        Color::Yellow
    } else {
        Color::Red
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Spell name
            Constraint::Length(1), // Incantation
            Constraint::Length(1), // Time bar
        ])
        .split(area);

    // Spell name header
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("✨ Casting: {} ", spell.name),
            Style::default().fg(element_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("({}mp)", spell.mp_cost),
            Style::default().fg(Color::Cyan),
        ),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Incantation to type
    let incant_para = Paragraph::new(Line::from(incantation_spans)).alignment(Alignment::Center);
    f.render_widget(incant_para, chunks[1]);

    // Time remaining gauge
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(time_color).bg(Color::DarkGray))
        .ratio(time_pct as f64)
        .label(format!("{:.1}s", time_remaining));
    f.render_widget(gauge, chunks[2]);
}

/// Render mana bar with spell cost preview
pub fn render_mana_bar_with_preview(
    f: &mut Frame,
    current_mp: i32,
    max_mp: i32,
    preview_cost: Option<i32>,
    area: Rect,
) {
    let mp_pct = current_mp as f64 / max_mp as f64;
    
    let (label, preview_style) = if let Some(cost) = preview_cost {
        let remaining = current_mp - cost;
        let can_afford = remaining >= 0;
        let color = if can_afford { Color::Yellow } else { Color::Red };
        (
            format!("MP: {}/{} (-{})", current_mp, max_mp, cost),
            Some(Style::default().fg(color)),
        )
    } else {
        (format!("MP: {}/{}", current_mp, max_mp), None)
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Blue).bg(Color::DarkGray))
        .ratio(mp_pct)
        .label(Span::styled(
            label,
            preview_style.unwrap_or_else(|| Style::default().fg(Color::Cyan)),
        ));

    f.render_widget(gauge, area);
}

/// Convert spell element to display color
fn element_to_color(element: &SpellElement) -> Color {
    match element {
        SpellElement::Physical => Color::White,
        SpellElement::Fire => Color::Red,
        SpellElement::Ice => Color::Cyan,
        SpellElement::Lightning => Color::Yellow,
        SpellElement::Arcane => Color::Magenta,
        SpellElement::Holy => Color::White,
        SpellElement::Dark => Color::Rgb(128, 0, 128),
        SpellElement::Nature => Color::Green,
    }
}

/// Get icon for spell element
fn element_icon(element: &SpellElement) -> &'static str {
    match element {
        SpellElement::Physical => "⚔",
        SpellElement::Fire => "🔥",
        SpellElement::Ice => "❄",
        SpellElement::Lightning => "⚡",
        SpellElement::Arcane => "✨",
        SpellElement::Holy => "✝",
        SpellElement::Dark => "🌑",
        SpellElement::Nature => "🌿",
    }
}

/// Get effect description for display
pub fn effect_description(effect: &SpellEffect) -> String {
    match effect {
        SpellEffect::Damage(dmg) => format!("Deal {} damage", dmg),
        SpellEffect::Heal(hp) => format!("Restore {} HP", hp),
        SpellEffect::Shield(amt) => format!("Gain {} shield", amt),
        SpellEffect::Buff { stat, amount, duration } => {
            format!("+{} {} for {} turns", amount, stat, duration)
        }
        SpellEffect::Debuff { stat, amount, duration } => {
            format!("-{} {} for {} turns", amount, stat, duration)
        }
        SpellEffect::Drain { damage, heal_percent } => {
            format!("Deal {} damage, heal {}%", damage, heal_percent)
        }
        SpellEffect::Multi { hits, damage_per_hit } => {
            format!("{} hits of {} damage", hits, damage_per_hit)
        }
        SpellEffect::Poison { damage, duration } => {
            format!("{} poison for {} turns", damage, duration)
        }
        SpellEffect::Stun { duration } => {
            format!("Stun for {} turns", duration)
        }
    }
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Compact spell info for tooltip
pub fn spell_tooltip(spell: &Spell) -> Vec<Line<'static>> {
    let element_color = element_to_color(&spell.element);
    let icon = element_icon(&spell.element);

    vec![
        Line::from(vec![
            Span::styled(icon.to_string(), Style::default().fg(element_color)),
            Span::raw(" "),
            Span::styled(
                spell.name.clone(),
                Style::default().fg(element_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            spell.description.clone(),
            Style::default().fg(Color::Gray),
        )),
        Line::from(vec![
            Span::styled("Cost: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}mp", spell.mp_cost), Style::default().fg(Color::Cyan)),
            Span::raw("  "),
            Span::styled("Power: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}", spell.base_power), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("Type: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("\"{}\"", spell.incantation),
                Style::default().fg(Color::Green),
            ),
            Span::styled(
                format!(" ({:.1}s)", spell.cast_time),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(Span::styled(
            effect_description(&spell.effect),
            Style::default().fg(Color::White),
        )),
    ]
}
