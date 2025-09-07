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
  ___ ___  _ __  _ __   ___  ___| |_ 
 / __/ _ \| '_ \| '_ \ / _ \/ __| __|
| (_| (_) | | | | | | |  __/ (__| |_ 
 \___\___/|_| |_|_| |_|\___|\___|\__|
"#;

pub fn render_menu(frame: &mut Frame, area: Rect, app: &mut App) {
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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(section_chunks[1]);

    frame.render_widget(title, section_chunks[0]);
    frame.render_widget(block, area);

    let input_area = center(
        chunks[0],
        Constraint::Percentage(70),
        Constraint::Percentage(80),
    );

    let input = Paragraph::new(app.msg_buffer.clone())
        .centered()
        .wrap(Wrap { trim: false });
    let input_block = Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .title(Line::from("Server IP").centered())
        .title_style(Color::Cyan);

    frame.render_widget(input.block(input_block), input_area);

    if let (Some(list), Some(list_state)) = (app.list.as_ref(), app.list_state.as_mut()) {
        let list_area = center(
            chunks[1],
            Constraint::Percentage(15),
            Constraint::Percentage(40),
        );

        let list = List::new(list.clone()).highlight_style(Style::new().fg(Color::Cyan).bold());

        frame.render_stateful_widget(list, list_area, list_state);
    }
}
