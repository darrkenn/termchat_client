use ratatui::crossterm::event::{self, KeyEvent};
use serde_json::json;
use tungstenite::Message;

use crate::app::App;

pub fn handle_connecting_key(key: KeyEvent, app: &mut App) {
    match key.code {
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
    }
}
