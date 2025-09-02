use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListDirection, ListItem, Paragraph},
};

pub fn render_message_area(
    frame: &mut Frame,
    area: Rect,
    messages: &[String],
    msg_buffer: &mut String,
) {
    let vertical_chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(90), Constraint::Percentage(10)],
    )
    .split(area);

    let messages: Vec<ListItem> = messages
        .iter()
        .map(|message| ListItem::from(message.as_str()))
        .collect();

    let message_list = List::new(messages)
        .style(Style::new().white())
        .direction(ListDirection::TopToBottom);

    let input_block = Block::bordered()
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::White));

    render_message_list(frame, message_list, vertical_chunks[0]);
    render_message_input(frame, msg_buffer, vertical_chunks[1]);
    frame.render_widget(input_block, vertical_chunks[1]);
}

fn render_message_list(frame: &mut Frame, messages: List, area: Rect) {
    let message_block = Block::bordered()
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::White));

    let inner_area = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Min(1)])
        .split(area)[0];

    frame.render_widget(message_block, area);
    frame.render_widget(messages, inner_area);
}

fn render_message_input(frame: &mut Frame, msg_buffer: &mut String, area: Rect) {
    let input_block = Block::bordered()
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::White));

    let inner_area = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Min(1)])
        .split(area)[0];

    let input = Paragraph::new(msg_buffer.as_str()).style(Style::default().fg(Color::White));

    frame.render_widget(input_block, area);
    frame.render_widget(input, inner_area);
}
