use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use serde_json::{Value, json};
use tungstenite::Message;

use crate::app::App;

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    if key.code == KeyCode::Esc {
        return true;
    }

    match key.code {
        KeyCode::Char(c) => {
            app.msg_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.msg_buffer.pop();
        }
        KeyCode::Enter => {
            let message = app.msg_buffer.trim();

            if !message.is_empty() {
                let writer = &app.socket_writer;

                let json_message = create_json_message(message);

                let message = Message::Text(json_message.to_string().into());
                _ = writer.try_send(message);
            }
            app.msg_buffer.clear();
        }
        _ => {}
    }

    false
}

fn create_json_message(message: &str) -> Value {
    if message.starts_with("/priv_msg") {
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
    }
}
