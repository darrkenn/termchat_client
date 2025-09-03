use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
};

use crate::{
    app::{App, Connect, Scene},
    render::{
        connect_area::render_connect_area, menu_area::render_menu_area,
        message_area::render_message_area, settings_area::render_settings_area,
    },
};

pub fn render(frame: &mut Frame, app: &mut App) {
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

    let scene = app.scene.clone();
    match scene {
        Scene::Menu => render_menu_area(frame, inner_area, app),
        Scene::Settings => render_settings_area(frame, inner_area),
        Scene::Connect(connect_scene) => render_connect_area(frame, inner_area, app, connect_scene),
        Scene::Message => {
            if let Some(server) = app.server.as_ref() {
                if let Some(messages) = &server.messages {
                    let msgs = messages.lock().unwrap();
                    render_message_area(frame, area, &msgs[..], &mut app.msg_buffer);
                }
            }
        }
    }
}

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
