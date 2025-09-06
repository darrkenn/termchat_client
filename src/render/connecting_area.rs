use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::{
    app::{App, Connection},
    render::render::center,
};

pub fn render_connecting(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::White));
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let input_area = center(
        chunks[0],
        Constraint::Percentage(70),
        Constraint::Percentage(80),
    );

    if let Some(connection_state) = app.connection_state.as_ref() {
        let mutex_guard = connection_state.lock().unwrap();
        let connection_state = mutex_guard.clone();
        match connection_state {
            Connection::Request(request) => match request.as_str() {
                "username" => {
                    let input = Paragraph::new(app.msg_buffer.clone())
                        .centered()
                        .wrap(Wrap { trim: false });
                    let input_block = input_block("username");
                    frame.render_widget(input.block(input_block), input_area);
                }
                "password" => {
                    app.msg_buffer.clear();
                    todo!()
                }
                _ => {}
            },
            Connection::Close => {
                todo!()
            }
            Connection::Connected => {
                todo!()
            }
            Connection::Error(e) => {
                todo!("{e}")
            }
            _ => {}
        }
    }
}

fn input_block(title: &str) -> Block<'_> {
    Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .title(Line::from(title).centered())
        .title_style(Color::Cyan)
}
