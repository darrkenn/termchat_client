use ratatui::crossterm::event::{self, KeyEvent, KeyEventKind};
use tokio::sync::mpsc;
use tungstenite::Message;

pub fn handle_key(key: KeyEvent, tx: &mpsc::Sender<Message>, msg_buffer: &mut String) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Enter => {
            if !msg_buffer.trim().is_empty() {
                let msg = Message::Text(msg_buffer.clone().into());
                let _ = tx.try_send(msg);
                msg_buffer.clear();
            }
        }
        event::KeyCode::Backspace => {
            msg_buffer.pop();
        }
        event::KeyCode::Char(char) => msg_buffer.push(char),
        _ => {}
    }
    false
}
