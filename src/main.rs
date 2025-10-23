mod app;
mod run;
mod websocket;
use std::{
    env, process,
    sync::{Arc, Mutex},
};

use futures::StreamExt;
use ratatui::{crossterm, widgets::ListItem};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::{
    app::{App, Connection, Scene},
    websocket::{websocket_reader, websocket_writer},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        let command = args[0].as_str();

        match command {
            "--connect" => {
                if args.len() > 1 {
                    let url = args[1].as_str();
                    connect(url.to_string());
                } else {
                    println!("termchat: Destination required")
                }
            }
            "--info" => {
                if args.len() > 1 {
                    let url = args[1].as_str();
                    get_info(url);
                }
            }
            _ => {}
        }
    } else {
        todo!()
    }

    //color_eyre::install()?;
    //crossterm::terminal::enable_raw_mode()?;
    //let terminal = ratatui::init();
    //_ = run::run(terminal, app);
    //ratatui::restore();
    //crossterm::terminal::disable_raw_mode()?;
    process::exit(0);
}

fn connect(url: String) {
    println!("Connecting to {url}");

    let (tx, rx) = mpsc::channel::<Message>(1);

    let mut app = App {
        scene: Scene::Menu,
        list_state: None,
        list: Some(Vec::<ListItem>::new()),
        connection_state: Arc::new(Mutex::new(Connection::None)),
        msg_buffer: "".to_string(),
        server: None,
        socket_writer: tx,
        messages: Arc::new(Mutex::new(Vec::<String>::new())),
    };

    tokio::spawn(async move {
        match connect_async(url.clone()).await {
            Ok((socket, _)) => {
                let messages_reader = Arc::clone(&app.messages);
                let (ws_w, ws_r) = socket.split();
                tokio::spawn(async move {
                    websocket_reader(messages_reader, ws_r, app.connection_state).await;
                });
                tokio::spawn(async move {
                    websocket_writer(ws_w, rx).await;
                });
            }
            Err(e) => {
                println!("Couldn't connect to url");
                process::exit(0)
            }
        }
    });
}

fn get_info(url: &str) {
    println!("termchat: Getting info from {url}")
}
