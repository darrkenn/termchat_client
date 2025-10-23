use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

use crate::{
    app::{App, Connection, Scene},
    render::{popup::render_popup, render::center},
};

pub fn render_connecting(frame: &mut Frame, area: Rect, app: &mut App) {
    let area = center(area, Constraint::Percentage(30), Constraint::Percentage(50));
    let input_area = center(area, Constraint::Percentage(70), Constraint::Percentage(10));

    let connection_state = app
        .connection_state
        .as_ref()
        .map(|cs| cs.lock().unwrap().clone());

    if let Some(connection_state) = connection_state {
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
                    let input = Paragraph::new(app.msg_buffer.clone())
                        .centered()
                        .wrap(Wrap { trim: false });
                    let input_block = input_block("password");
                    frame.render_widget(input.block(input_block), input_area);
                }
                _ => {}
            },
            Connection::Connected => {
                app.scene = Scene::Message;
            }
            Connection::Error(e) => {
                render_popup(frame, area, &e);
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
