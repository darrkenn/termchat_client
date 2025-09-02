use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};
use std::time::Duration;

use crate::{app::App, keyhandling::handle_key, render::render::render};

pub fn run(mut terminal: DefaultTerminal, mut app: App) -> color_eyre::Result<()> {
    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if handle_key(key, &mut app) {
                    break;
                }
            }
        }

        terminal.draw(|f| render(f, &mut app))?;
    }
    Ok(())
}
