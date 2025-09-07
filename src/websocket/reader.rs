use std::{
    panic,
    sync::{Arc, Mutex},
};

use futures::{StreamExt, stream::SplitStream};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

use crate::app::Connection;

type Messages = Arc<Mutex<Vec<String>>>;

pub async fn websocket_reader(
    messages: Messages,
    mut ws_r: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    connection_state: Arc<Mutex<Connection>>,
) {
    while let Some(msg) = ws_r.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    match json["type"].as_str() {
                        Some("request") => match json["reason"].as_str() {
                            Some("username") => {
                                let mut conn = connection_state.lock().unwrap();
                                *conn = Connection::Request("username".to_string());
                            }
                            Some("password") => {
                                let mut conn = connection_state.lock().unwrap();
                                *conn = Connection::Request("password".to_string());
                            }
                            _ => {}
                        },
                        Some("server") => match json["reason"].as_str() {
                            Some("authenticated") => {
                                let mut conn = connection_state.lock().unwrap();
                                *conn = Connection::Connected;
                            }
                            Some("not-authenticated") => {
                                let mut conn = connection_state.lock().unwrap();
                                *conn = Connection::Error("Could not be authenticated".to_string());
                            }
                            _ => {}
                        },
                        Some("message") => {
                            if let Some(contents) = json["from"].as_str() {
                                todo!()
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(Message::Close(_)) => {
                panic!("Closed");
            }
            Ok(ok) => {
                panic!("{ok}");
            }
            Err(e) => {
                panic!("{e}");
            }
        }
    }
}
