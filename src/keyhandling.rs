use ratatui::crossterm::event::{self, KeyEvent, KeyEventKind};

use crate::app::{App, Connect, Scene};

const MENU_OPTIONS: [&str; 3] = ["Connect", "Saved", "Settings"];
const CONNECT_OPTIONS: [&str; 2] = ["Connect", "Info"];

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

    let scene = app.scene.clone();
    match scene {
        Scene::Menu => handle_menu_key(key, app),
        Scene::Connect(connect_scene) => handle_connect_key(key, app, connect_scene),
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

fn handle_connect_key(key: KeyEvent, app: &mut App, connect_scene: Connect) {
    match connect_scene {
        Connect::Menu => match key.code {
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
                    let selection = CONNECT_OPTIONS[i.selected().unwrap()];
                    match selection {
                        "Connect" => {
                            app.scene = Scene::Connect(Connect::Connecting);
                            app.update_scene();
                        }
                        "Info" => {
                            app.scene = Scene::Connect(Connect::Info);
                            app.update_scene();
                        }
                        _ => {}
                    }
                }
            }
            event::KeyCode::Char(c) => {
                app.msg_buffer.push(c);
            }
            event::KeyCode::Backspace => {
                app.msg_buffer.pop();
            }
            _ => {}
        },
        Connect::Info => match key.code {
            event::KeyCode::Char('c') => {
                app.scene = Scene::Connect(Connect::Connecting);
                app.update_scene();
            }
            event::KeyCode::Enter => {
                app.scene = Scene::Connect(Connect::Menu);
                app.update_scene();
            }
            _ => {}
        },
        Connect::Connecting => {}
    }
}
