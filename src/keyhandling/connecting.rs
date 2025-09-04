use ratatui::crossterm::event::{self, KeyEvent};

use crate::app::App;

pub fn handle_connecting_key(key: KeyEvent, app: &mut App) {
    match key.code {
        event::KeyCode::Char(c) => app.msg_buffer.push(c),
        event::KeyCode::Backspace => {
            app.msg_buffer.pop();
        }
        event::KeyCode::Enter => {
            panic!("{}", app.msg_buffer.clone())
        }
        _ => {}
    }
}
