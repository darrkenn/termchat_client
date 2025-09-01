use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};
use std::time::Duration;
use tokio::sync::mpsc;
use tungstenite::Message;

use crate::{Messages, keyhandling::handle_key, render::render::render};

pub fn run(
    mut terminal: DefaultTerminal,
    tx: mpsc::Sender<Message>,
    messages: &Messages,
    msg_buffer: &mut String,
) -> color_eyre::Result<()> {
    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if handle_key(key, &tx, msg_buffer) {
                    break;
                }
            }
        }
        terminal.draw(|f| render(f, messages, msg_buffer))?;
    }
    Ok(())
}
