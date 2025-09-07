use ratatui::crossterm::event::{self, KeyEvent, KeyEventKind};

use crate::{
    app::{App, Scene},
    keyhandling::{
        connect::handle_connect_key, connecting::handle_connecting_key, menu::handle_menu_key,
        message::handle_message_key, settings::handle_settings_key,
    },
};

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    if key.code == event::KeyCode::Esc {
        return true;
    }

    let scene = app.scene.clone();
    match scene {
        Scene::Menu => handle_menu_key(key, app),
        Scene::Connect(connect_scene) => handle_connect_key(key, app, connect_scene),
        Scene::Connecting => handle_connecting_key(key, app),
        Scene::Message => handle_message_key(key, app),
        Scene::Settings => handle_settings_key(key),
    }

    false
}
