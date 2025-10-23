use ratatui::crossterm::event::{self, KeyEvent};
use serde_json::json;
use tungstenite::Message;

use crate::app::App;

pub fn handle_message_key(key: KeyEvent, app: &mut App) {
    match key.code {
        event::KeyCode::Char(c) => {
            app.msg_buffer.push(c);
        }
        event::KeyCode::Backspace => {
            app.msg_buffer.pop();
        }
        event::KeyCode::Enter => {
            let message = app.msg_buffer.trim();

            if !message.is_empty() {
                if let Some(writer) = &app.socket_writer {
                    let json = if message.starts_with("/priv_msg") {
                        let mut parts = message.splitn(3, " ");
                        parts.next();

                        let receiver = parts.next();
                        let message = parts.next();

                        if let (Some(receiver), Some(message)) = (receiver, message) {
                            json!({
                                "type": "priv_msg",
                                "receiver": receiver,
                                "message": message
                            })
                        } else {
                            panic!("Invalid priv_msg");
                        }
                    } else {
                        json!({
                            "type": "message",
                            "value": message
                        })
                    };

                    let message = Message::Text(json.to_string().into());
                    _ = writer.try_send(message);
                }
                app.msg_buffer.clear();
            }
        }
        _ => {}
    }
}
