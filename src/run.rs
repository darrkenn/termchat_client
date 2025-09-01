use std::time::Duration;

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};

use crate::{keyhandling::handle_key, render::render::render};

pub fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if handle_key(key) {
                    break;
                }
            }
        }
        terminal.draw(|f| render(f))?;
    }
    Ok(())
}
