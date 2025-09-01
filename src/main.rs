mod keyhandling;
mod render;
mod run;
use std::process;

use ratatui::crossterm;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://localhost:3113/chat";

    let (mut socket, _) = connect_async(url)
        .await
        .expect("Couldn't connect to websocket");

    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let _ = run::run(terminal);
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    process::exit(0);
}
