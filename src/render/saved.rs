use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, List, Paragraph, Wrap},
};

use crate::{app::App, render::render::center};

const ASCII: &str = r#"
                         _ 
                        | |
 ___  __ ___   _____  __| |
/ __|/ _` \ \ / / _ \/ _` |
\__ \ (_| |\ V /  __/ (_| |
|___/\__,_| \_/ \___|\__,_|
"#;

pub fn render_saved_area(frame: &mut Frame, area: Rect, app: &mut App) {
    let area = center(area, Constraint::Percentage(30), Constraint::Percentage(50));

    let title = Paragraph::new(ASCII)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);

    let block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::White));

    let section_chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(40), Constraint::Percentage(60)],
    )
    .split(area);

    frame.render_widget(title, section_chunks[0]);
    frame.render_widget(block, area);

    if let (Some(list), Some(list_state)) = (app.list.as_ref(), app.list_state.as_mut()) {
        let list_area = center(
            section_chunks[1],
            Constraint::Percentage(100),
            Constraint::Percentage(40),
        );

        let list = List::new(list.clone()).highlight_style(Style::new().fg(Color::Cyan).bold());

        frame.render_stateful_widget(list, list_area, list_state);
    }
}
