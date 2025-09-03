use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, List, Paragraph},
};

use crate::{app::App, render::render::center};

const ASCII: &str = r#"
 _                           _           _   
| |                         | |         | |  
| |_ ___ _ __ _ __ ___   ___| |__   __ _| |_ 
| __/ _ \ '__| '_ ` _ \ / __| '_ \ / _` | __|
| ||  __/ |  | | | | | | (__| | | | (_| | |_ 
 \__\___|_|  |_| |_| |_|\___|_| |_|\__,_|\__|
"#;

pub fn render_menu_area(frame: &mut Frame, area: Rect, app: &mut App) {
    let area = center(area, Constraint::Percentage(30), Constraint::Percentage(40));

    let title = Paragraph::new(ASCII)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);

    let block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::White));

    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(40), Constraint::Percentage(60)],
    )
    .split(area);

    frame.render_widget(title, chunks[0]);
    frame.render_widget(block, area);

    if let (Some(list), Some(list_state)) = (app.list.as_ref(), app.list_state.as_mut()) {
        let list_area = center(
            chunks[1],
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        );

        let list = List::new(list.clone()).highlight_style(Style::new().fg(Color::Cyan).bold());

        frame.render_stateful_widget(list, list_area, list_state);
    }
}
