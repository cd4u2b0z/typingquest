//! Lore and milestone rendering functions

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    Frame,
};
use crate::game::state::GameState;
use super::theme::Palette;

/// Render a lore discovery popup - atmospheric and mysterious
pub fn render_lore_discovery(f: &mut Frame, state: &GameState) {
    let area = f.area();
    let bg = Block::default().style(Style::default().bg(Color::Rgb(10, 10, 15)));
    f.render_widget(bg, area);
    
    let popup_width = area.width.min(70);
    let popup_height = area.height.min(20);
    let popup_area = Rect::new((area.width - popup_width) / 2, (area.height - popup_height) / 2, popup_width, popup_height);
    let clear = Block::default().style(Style::default().bg(Palette::BG_PANEL));
    f.render_widget(clear, popup_area);
    
    if let Some((title, content)) = &state.current_lore {
        let chunks = Layout::default().direction(Direction::Vertical).margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(8), Constraint::Length(3)]).split(popup_area);
        
        let title_text = format!("󰈙 LORE DISCOVERED: {} 󰈙", title);
        let title_widget = Paragraph::new(title_text)
            .style(Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Rgb(255, 215, 0))).border_type(BorderType::Double));
        f.render_widget(title_widget, chunks[0]);
        
        let content_widget = Paragraph::new(content.clone())
            .style(Style::default().fg(Palette::TEXT).add_modifier(Modifier::ITALIC))
            .alignment(Alignment::Left).wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Palette::TEXT_DIM)).padding(Padding::horizontal(1)));
        f.render_widget(content_widget, chunks[1]);
        
        let hint = Paragraph::new("[ Press any key to continue ]").style(Style::default().fg(Palette::TEXT_DIM)).alignment(Alignment::Center);
        f.render_widget(hint, chunks[2]);
    }
}

/// Render a milestone story event - dramatic and important
pub fn render_milestone(f: &mut Frame, state: &GameState) {
    let area = f.area();
    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 5, 10)));
    f.render_widget(bg, area);
    
    let popup_width = area.width.min(80);
    let popup_height = area.height.min(24);
    let popup_area = Rect::new((area.width - popup_width) / 2, (area.height - popup_height) / 2, popup_width, popup_height);
    
    if let Some(milestone_text) = &state.current_milestone {
        let chunks = Layout::default().direction(Direction::Vertical).margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(12), Constraint::Length(3)]).split(popup_area);
        
        let floor = state.get_current_floor();
        let title = format!("═══ FLOOR {} MILESTONE ═══", floor);
        let title_widget = Paragraph::new(title)
            .style(Style::default().fg(Color::Rgb(220, 20, 60)).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Rgb(139, 0, 0))).border_type(BorderType::Thick));
        f.render_widget(title_widget, chunks[0]);
        
        let content_widget = Paragraph::new(milestone_text.clone())
            .style(Style::default().fg(Palette::TEXT))
            .alignment(Alignment::Center).wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Rgb(100, 0, 0))).padding(Padding::uniform(1)));
        f.render_widget(content_widget, chunks[1]);
        
        let hint = Paragraph::new("[ Press ENTER to face your destiny ]")
            .style(Style::default().fg(Color::Rgb(220, 20, 60)).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(hint, chunks[2]);
    }
}
