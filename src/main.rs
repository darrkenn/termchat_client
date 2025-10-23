mod app;
mod keyhandling;
mod render;
mod run;
mod websocket;
use std::process;

use ratatui::{crossterm, widgets::ListItem};

use crate::app::{App, Scene};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App {
        scene: Scene::Menu,
        list_state: None,
        list: Some(Vec::<ListItem>::new()),
        connection_state: None,
        msg_buffer: "".to_string(),
        server: None,
        socket_writer: None,
    };

    app.update_scene();

    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    _ = run::run(terminal, app);
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    process::exit(0);
}
