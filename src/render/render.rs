use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{Messages, render::message_area::render_message_area};

pub fn render(frame: &mut Frame, messages: &Messages, msg_buffer: &mut String) {
    let area = frame.area();

    let inner_area = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Min(1)])
        .split(area)[0];

    let vertical_chunks = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(60), Constraint::Percentage(40)],
    )
    .split(inner_area);

    let msgs = messages.lock().unwrap();

    render_message_area(frame, &msgs[..], vertical_chunks[0], msg_buffer);
}
