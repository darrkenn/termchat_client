use std::sync::{Arc, Mutex};

use ratatui::widgets::ListState;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use tokio::sync::mpsc;
use tungstenite::Message;

#[derive(Clone, Debug)]
pub enum Connection {
    Connected,
    Request(String),
    Error(String),
    Close,
    None,
}

pub struct App {
    pub list_state: ListState,
    pub connection_state: Arc<Mutex<Connection>>,
    pub msg_buffer: String,
    pub socket_writer: mpsc::Sender<Message>,
    pub messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Clone, Debug)]
pub struct Server {
    pub info: Option<Arc<Mutex<Info>>>,
    pub messages: Option<Arc<Mutex<Vec<String>>>>,
    pub ip: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub description: String,
    pub language: String,
    pub tags: Vec<String>,
}

async fn make_info_request(server_info: Arc<Mutex<Info>>, ip: String) {
    let response = match reqwest::get(&ip).await {
        Ok(response) => response,
        Err(_) => {
            return;
        }
    };

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("");

    if content_type.contains("application/json") {
        match response.text().await {
            Ok(body) => match serde_json::from_str::<Info>(&body) {
                Ok(info) => {
                    let mut server_info = server_info.lock().unwrap();
                    *server_info = info;
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }
}
