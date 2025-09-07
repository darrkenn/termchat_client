use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::App;

pub fn render_info(frame: &mut Frame, area: Rect, app: &mut App) {
    let info = match &app.server {
        Some(server) => match &server.info {
            Some(info) => match info.lock() {
                Ok(mutex_guard) => Some(mutex_guard),
                Err(_) => None,
            },
            None => None,
        },
        None => None,
    };

    if let Some(info) = info {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(6),
                Constraint::Length(4),
                Constraint::Length(3),
            ])
            .split(area);

        let name = Paragraph::new(info.name.clone())
            .block(
                Block::default()
                    .title(Line::from(Span::styled(
                        "Name",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(name, chunks[0]);
        let description = Paragraph::new(info.description.clone())
            .block(
                Block::default()
                    .title(Line::from(Span::styled(
                        "Description",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(Wrap { trim: true });
        frame.render_widget(description, chunks[1]);
        let tags = Paragraph::new(info.tags.join(",")).block(
            Block::default()
                .title(Line::from(Span::styled(
                    "Tags",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(tags, chunks[2]);
        let language = Paragraph::new(info.language.clone()).block(
            Block::default()
                .title(Line::from(Span::styled(
                    "Language",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(language, chunks[3]);
    }
}
