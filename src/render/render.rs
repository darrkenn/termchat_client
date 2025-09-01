use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, BorderType},
};

pub fn render(frame: &mut Frame) {
    let area = frame.area();

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Chat room")
        .title_alignment(ratatui::layout::HorizontalAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(block, area);
}
