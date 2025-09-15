use std::fs;

use ratatui::crossterm::event::{self, KeyEvent};
use serde_json::Value;

use crate::{app::App, keyhandling::connect::handle_connection};

pub fn handle_saved_key(key: KeyEvent, app: &mut App) {
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
            if let Some(i) = app.list_state {
                if let Ok(data) = fs::read_to_string("/etc/termchat/client/servers.json") {
                    if let Ok(value) = serde_json::from_str::<Value>(&data) {
                        let servers: Vec<String> = serde_json::from_value(value["servers"].clone())
                            .expect("Couldnt get servers");
                        let ip = servers.get(i.selected().unwrap()).unwrap();
                        app.msg_buffer.clear();
                        app.msg_buffer = ip.clone();
                        handle_connection(app);
                        app.msg_buffer.clear();
                    }
                }
            }
        }
        _ => {}
    }
}
