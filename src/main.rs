mod app;
mod keyhandling;
mod render;
mod run;
mod websocket;
use core::time;
use std::{
    env,
    io::{self, Write},
    process,
    sync::{Arc, Mutex},
};

use futures::StreamExt;
use ratatui::{crossterm, widgets::ListState};
use serde_json::json;
use tokio::sync::mpsc::{self, Sender};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::{
    app::{App, Connection},
    run::run,
    websocket::{websocket_reader, websocket_writer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        let command = args[0].as_str();

        match command {
            "--connect" => {
                if args.len() > 1 {
                    let url = args[1].as_str();
                    println!("Connecting to {url}");

                    let mut app = connect(get_url(url.to_string())).await;
                    check_connection(&mut app);
                    color_eyre::install()?;
                    crossterm::terminal::enable_raw_mode()?;
                    let terminal = ratatui::init();
                    _ = run(terminal, app);
                    ratatui::restore();
                    crossterm::terminal::disable_raw_mode()?;
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

    process::exit(0);
}

async fn connect(url: String) -> App {
    let (tx, rx) = mpsc::channel::<Message>(1);

    let app = App {
        list_state: ListState::default(),
        connection_state: Arc::new(Mutex::new(Connection::None)),
        msg_buffer: "".to_string(),
        socket_writer: tx,
        messages: Arc::new(Mutex::new(Vec::<String>::new())),
    };

    let messages_reader = Arc::clone(&app.messages);
    let connection_state = Arc::clone(&app.connection_state);

    tokio::spawn(async move {
        match connect_async(url.clone()).await {
            Ok((socket, _)) => {
                let (ws_w, ws_r) = socket.split();
                tokio::spawn(async move {
                    websocket_reader(messages_reader, ws_r, connection_state).await;
                });
                tokio::spawn(async move {
                    websocket_writer(ws_w, rx).await;
                });
            }
            Err(e) => {
                let mut connection_state = connection_state.lock().unwrap();
                *connection_state = Connection::Error(e.to_string());
            }
        }
    });

    app
}

fn check_connection(app: &mut App) {
    loop {
        let connection_state = app.connection_state.lock().unwrap().clone();
        match connection_state {
            Connection::Request(ref r) => match r.as_str() {
                "username" => {
                    let mut username = String::new();
                    let writer = &app.socket_writer;
                    print!("Username: ");
                    io::stdout().flush().expect("Couldnt flush stdout");
                    io::stdin()
                        .read_line(&mut username)
                        .expect("Couldnt read stdin");
                    send_login(username.trim().to_string(), writer);
                    std::thread::sleep(time::Duration::from_millis(100));
                }
                "password" => {
                    let mut password = String::new();
                    let writer = &app.socket_writer;
                    print!("Password: ");
                    io::stdout().flush().expect("Couldnt flush stdout");
                    io::stdin()
                        .read_line(&mut password)
                        .expect("Couldnt read stdin");
                    send_login(password.trim().to_string(), writer);
                    std::thread::sleep(time::Duration::from_millis(100));
                    break;
                }
                _ => {
                    print!("h")
                }
            },
            Connection::Error(e) => {
                eprintln!("termchat: error: {e}");
                process::exit(1);
            }
            Connection::Close => {
                println!("termchat: connection closed");
                process::exit(0);
            }
            Connection::Connected => {
                break;
            }
            _ => {}
        }
        std::thread::sleep(time::Duration::from_millis(100));
    }
}

fn get_info(url: &str) {
    println!("termchat: Getting info from {url}")
}

fn get_url(mut url: String) -> String {
    if url.starts_with("ws://") || url.starts_with("wss://") {
    } else if url.starts_with("http://") {
        url = url.replace("http://", "ws://");
    } else if url.starts_with("https://") {
        url = url.replace("https://", "wss://");
    } else {
        url = format!("ws://{url}");
    }
    if url.ends_with("/") {
        url.push_str("chat");
        url
    } else if !url.ends_with("/chat") {
        url.push_str("/chat");
        url
    } else {
        url
    }
}

fn send_login(data: String, writer: &Sender<Message>) {
    let json_message = json!({
        "type": "response",
        "value": data
    });

    let message = Message::Text(json_message.to_string().into());

    _ = writer.try_send(message);
}
