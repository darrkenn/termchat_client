use std::sync::Arc;

use futures::StreamExt;
use ratatui::crossterm::event::{self, KeyEvent};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::{
    app::{App, Connect, Scene},
    websocket::{websocket_reader, websocket_writer},
};

const CONNECT_OPTIONS: [&str; 2] = ["Connect", "Info"];

pub fn handle_connect_key(key: KeyEvent, app: &mut App, connect_scene: Connect) {
    match connect_scene {
        Connect::Menu => match key.code {
            event::KeyCode::Up => {
                if let Some(list_state) = app.list_state.as_mut() {
                    list_state.select_previous();
                }
            }
            event::KeyCode::Down => {
                if let Some(list_state) = app.list_state.as_mut() {
                    list_state.select_next();
                }
            }
            event::KeyCode::Enter => {
                if let Some(i) = app.list_state.as_ref() {
                    let selection = CONNECT_OPTIONS[i.selected().unwrap()];
                    match selection {
                        "Connect" => {
                            handle_connection(app);
                            app.msg_buffer.clear();
                        }
                        "Info" => {
                            app.scene = Scene::Connect(Connect::Info);
                            app.update_scene();
                        }
                        _ => {}
                    }
                }
            }
            event::KeyCode::Char(c) => {
                app.msg_buffer.push(c);
            }
            event::KeyCode::Backspace => {
                app.msg_buffer.pop();
            }
            _ => {}
        },
        Connect::Info => match key.code {
            event::KeyCode::Char('c') => {
                app.scene = Scene::Connecting;
                app.update_scene();
            }
            event::KeyCode::Enter => {
                app.scene = Scene::Connect(Connect::Menu);
                app.update_scene();
            }
            _ => {}
        },
    }
}

fn get_url(mut msg_buffer: String) -> String {
    if msg_buffer.starts_with("ws://") || msg_buffer.starts_with("wss://") {
    } else if msg_buffer.starts_with("http://") {
        msg_buffer = msg_buffer.replace("http://", "ws://");
    } else if msg_buffer.starts_with("https://") {
        msg_buffer = msg_buffer.replace("https://", "wss://");
    };

    if msg_buffer.ends_with("/") {
        msg_buffer.push_str("chat");
        msg_buffer
    } else if !msg_buffer.ends_with("/chat") {
        msg_buffer.push_str("/chat");
        msg_buffer
    } else {
        msg_buffer
    }
}

pub fn handle_connection(app: &mut App) {
    app.scene = Scene::Connecting;
    app.update_scene();

    let url = get_url(app.msg_buffer.clone());
    let server = app.server.clone();
    let connection_state = app.connection_state.clone();

    let (tx, rx) = mpsc::channel::<Message>(12);
    app.socket_writer = Some(tx);

    tokio::spawn(async move {
        match connect_async(url).await {
            Ok((socket, _)) => {
                let (ws_w, ws_r) = socket.split();

                if let (Some(server), Some(connection_state)) = (server, connection_state) {
                    if let Some(messages) = &server.messages {
                        let messages_reader = Arc::clone(messages);
                        tokio::spawn(async move {
                            websocket_reader(messages_reader, ws_r, connection_state).await;
                        });
                        tokio::spawn(async move {
                            websocket_writer(ws_w, rx).await;
                        });
                    };
                }
            }
            Err(e) => {
                eprintln!("{e}")
            }
        }
    });
}
