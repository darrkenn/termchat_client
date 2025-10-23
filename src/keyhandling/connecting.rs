use ratatui::crossterm::event::{self, KeyEvent};
use serde_json::json;
use tungstenite::Message;

use crate::app::{App, Connection, Scene};

pub fn handle_connecting_key(key: KeyEvent, app: &mut App) {
    let connection_state = app
        .connection_state
        .as_ref()
        .map(|cs| cs.lock().unwrap().clone());
    if let Some(connection_state) = connection_state {
        match connection_state {
            Connection::Request(_) => match key.code {
                event::KeyCode::Char(c) => app.msg_buffer.push(c),
                event::KeyCode::Backspace => {
                    app.msg_buffer.pop();
                }
                event::KeyCode::Enter => {
                    let message = app.msg_buffer.trim();

                    if !message.is_empty() {
                        if let Some(writer) = &app.socket_writer {
                            let json_message = json!({
                                "type": "response",
                                "value": message,
                            });

                            let message = Message::Text(json_message.to_string().into());
                            _ = writer.try_send(message);
                        }
                        app.msg_buffer.clear();
                    }
                }
                _ => {}
            },
            Connection::Error(_) => {
                if key.code == event::KeyCode::Enter {
                    app.scene = Scene::Menu;
                    app.msg_buffer.clear();
                    app.update_scene();
                }
            }
            _ => {}
        }
    }
}
