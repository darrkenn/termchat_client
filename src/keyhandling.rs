use ratatui::crossterm::event::{self, KeyEvent, KeyEventKind};

pub fn handle_key(key: KeyEvent) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        _ => {}
    }
    false
}
