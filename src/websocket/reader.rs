use std::sync::{Arc, Mutex};

use futures::{StreamExt, stream::SplitStream};
use log::info;
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
    info!(target: "reader", "Started thread");
    while let Some(msg) = ws_r.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    match json["type"].as_str() {
                        Some("request") => match json["reason"].as_str() {
                            Some("username") => {
                                info!(target: "reader", "Received request for username");
                                let mut conn = connection_state.lock().unwrap();
                                *conn = Connection::Request("username".to_string());
                                match *conn {
                                    Connection::Request(_) => {
                                        info!(target: "reader", "Connection state is request")
                                    }
                                    Connection::None => {
                                        info!(target: "reader", "Connection state is none")
                                    }
                                    Connection::Connecting => {
                                        info!(target: "reader", "Connection state is connecting")
                                    }
                                    _ => {}
                                }
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
                            Some("unauthenticated") => match json["message"].as_str() {
                                Some("Incorrect password") => {
                                    let mut conn = connection_state.lock().unwrap();
                                    *conn = Connection::Error("Incorrect password".to_string());
                                }
                                _ => {}
                            },
                            Some("message") => {
                                let mut msgs = messages.lock().unwrap();
                                let message =
                                    format!("[SERVER]:{}", json["body"].as_str().unwrap());
                                msgs.push(message);
                            }
                            Some("clear") => {
                                let mut msgs = messages.lock().unwrap();
                                msgs.clear();
                            }
                            _ => {}
                        },
                        Some("message") => {
                            let mut msgs = messages.lock().unwrap();
                            let message = format!(
                                "[{}]:{}",
                                json["from"].as_str().unwrap_or(""),
                                json["body"].as_str().unwrap_or("")
                            );
                            msgs.push(message);
                        }
                        Some("priv_msg") => {
                            let mut msgs = messages.lock().unwrap();
                            let message = format!(
                                "Private message from [{}]: {}",
                                json["sender"].as_str().unwrap_or("???"),
                                json["body"].as_str().unwrap_or("???")
                            );
                            msgs.push(message)
                        }
                        _ => {}
                    }
                }
            }
            Ok(Message::Close(_)) => {
                let mut conn = connection_state.lock().unwrap();
                *conn = Connection::Close;
                break;
            }
            Ok(_) => {
                let mut conn = connection_state.lock().unwrap();
                *conn = Connection::Close;
            }
            Err(_) => {
                let mut conn = connection_state.lock().unwrap();
                *conn = Connection::Close;
            }
        }
    }
}
