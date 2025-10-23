use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, HorizontalAlignment, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph, Wrap},
};

use crate::render::render::center;

pub fn render_popup(frame: &mut Frame, area: Rect, reason: &str) {
    let area = center(area, Constraint::Percentage(80), Constraint::Percentage(20));
    frame.render_widget(Clear, area);
    let block = Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .title("Popup")
        .title_alignment(HorizontalAlignment::Center)
        .title_style(Style::default().fg(Color::Cyan).bold());

    let inner_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1)])
        .margin(1)
        .split(area)[0];

    let reason = Paragraph::new(reason)
        .style(Style::default().fg(Color::Red).bold())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(block, area);
    frame.render_widget(reason, inner_area);
}
