mod app;
mod keyhandling;
mod render;
mod run;
mod websocket;
use std::{
    process,
    sync::{Arc, Mutex},
};

use ratatui::{
    crossterm,
    widgets::{ListItem, ListState},
};

use crate::app::{App, Scene};

type Messages = Arc<Mutex<Vec<String>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App {
        scene: Scene::Menu,
        list_state: Some(ListState::default()),
        list: Some(Vec::<ListItem>::new()),
        connection_state: None,
        msg_buffer: "".to_string(),
        messages: None,
    };

    app.update_list();

    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let _ = run::run(terminal, app);
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    process::exit(0);
}
