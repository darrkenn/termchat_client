use ratatui::crossterm::event::{self, KeyEvent};

use crate::app::{App, Connect, Scene};

const MENU_OPTIONS: [&str; 3] = ["Connect", "Saved", "Settings"];

pub fn handle_menu_key(key: KeyEvent, app: &mut App) {
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
        event::KeyCode::Enter => {
            if let Some(i) = app.list_state.as_ref() {
                let selection = MENU_OPTIONS[i.selected().unwrap()];
                match selection {
                    "Connect" => {
                        app.scene = Scene::Connect(Connect::Menu);
                        app.update_scene();
                    }
                    "Settings" => {
                        app.scene = Scene::Settings;
                        app.update_scene();
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
