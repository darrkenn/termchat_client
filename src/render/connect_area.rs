use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, Paragraph, Wrap},
};

use crate::{
    app::{App, Connect},
    render::render::center,
};

const ASCII: &str = r#"
                                 _   
                                | |  
  ___ ___  _ __  _ __   ___  ___| |_ 
 / __/ _ \| '_ \| '_ \ / _ \/ __| __|
| (_| (_) | | | | | | |  __/ (__| |_ 
 \___\___/|_| |_|_| |_|\___|\___|\__|
"#;

pub fn render_connect_area(frame: &mut Frame, area: Rect, app: &mut App, connect_scene: Connect) {
    let area = center(area, Constraint::Percentage(30), Constraint::Percentage(40));
    match connect_scene {
        Connect::Connecting => {
            render_connect_connecting(frame, area, app);
        }
        Connect::Info => {
            render_connect_info(frame, area, app);
        }
        Connect::Menu => {
            render_connect_menu(frame, area, app);
        }
    }
}

fn render_connect_connecting(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::White));
    frame.render_widget(block, area);
}

fn render_connect_info(frame: &mut Frame, area: Rect, app: &mut App) {
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

fn render_connect_menu(frame: &mut Frame, area: Rect, app: &mut App) {
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
