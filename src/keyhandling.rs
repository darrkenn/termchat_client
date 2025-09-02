use ratatui::{
    crossterm::event::{self, KeyEvent, KeyEventKind},
    widgets::ListState,
};

use crate::app::{App, Scene};

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        _ => {}
    }

    match app.scene {
        Scene::Menu => handle_menu_key(key, app),
        Scene::Message => handle_message_key(key),
        Scene::Settings => handle_settings_key(key),
    }

    false
}

fn handle_menu_key(key: KeyEvent, app: &mut App) {
    match key.code {
        event::KeyCode::Up => {
            if let Some(list_state) = app.list_state.as_mut() {
                list_state.select_previous();
            }
        }
        event::KeyCode::Down => {
            if let Some(list_state) = app.list_state.as_mut() {
                list_state.select_next();
            }
        }
        _ => {}
    }
}

fn handle_message_key(key: KeyEvent) {
    match key.code {
        _ => {}
    }
}

fn handle_settings_key(key: KeyEvent) {
    match key.code {
        _ => {}
    }
}
