mod keyhandling;
mod render;
mod run;
mod websocket;
use std::{
    process,
    sync::{Arc, Mutex},
};

use futures::StreamExt;
use ratatui::crossterm;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::websocket::{websocket_reader, websocket_writer};

type Messages = Arc<Mutex<Vec<String>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://localhost:3113/chat";

    let (socket, _) = connect_async(url).await?;

    let (ws_w, ws_r) = socket.split();

    let messages = Arc::new(Mutex::new(Vec::<String>::new()));

    let messages_reader = Arc::clone(&messages);

    tokio::spawn(async move { websocket_reader(messages_reader, ws_r).await });

    let (tx, rx) = mpsc::channel::<Message>(12);

    tokio::spawn(async move { websocket_writer(ws_w, rx).await });

    let mut msg_buffer: String = String::new();

    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let _ = run::run(terminal, tx, &messages, &mut msg_buffer);
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    process::exit(0);
}
