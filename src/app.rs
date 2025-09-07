use std::sync::{Arc, Mutex};

use ratatui::{
    layout::Alignment,
    text::Line,
    widgets::{ListItem, ListState},
};
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use tokio::sync::mpsc;
use tungstenite::Message;

#[derive(Clone, Debug)]
pub enum Connection {
    Connected,
    Connecting,
    Request(String),
    Error(String),
    Close,
}

#[derive(Clone)]
pub enum Scene {
    Menu,
    Settings,
    Connect(Connect),
    Connecting,
    Message,
}
#[derive(Clone)]
pub enum Connect {
    Menu,
    Info,
}

pub struct App<'a> {
    pub scene: Scene,
    pub list_state: Option<ListState>,
    pub list: Option<Vec<ListItem<'a>>>,
    pub connection_state: Option<Arc<Mutex<Connection>>>,
    pub msg_buffer: String,
    pub server: Option<Server>,
    pub socket_writer: Option<mpsc::Sender<Message>>,
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

impl App<'_> {
    pub fn update_scene(&mut self) {
        match &self.scene {
            Scene::Menu => {
                self.list = Some(vec![
                    ListItem::from(Line::from(" Connect ").alignment(Alignment::Center)),
                    ListItem::from(Line::from("Saved ").alignment(Alignment::Center)),
                    ListItem::from(Line::from(" Settings").alignment(Alignment::Center)),
                ]);
                if let Some(list_state) = self.list_state.as_mut() {
                    list_state.select_first();
                };
                self.server = None;
            }
            Scene::Settings => {
                self.list = Some(vec![ListItem::from(
                    Line::from("Manage saved").alignment(Alignment::Center),
                )]);
                if let Some(list_state) = self.list_state.as_mut() {
                    list_state.select_first();
                };
            }
            Scene::Connect(connect_scene) => match connect_scene {
                Connect::Menu => {
                    self.list = Some(vec![
                        ListItem::from(Line::from("Connect").alignment(Alignment::Center)),
                        ListItem::from(Line::from("Info  ").alignment(Alignment::Center)),
                    ]);
                    if let Some(list_state) = self.list_state.as_mut() {
                        list_state.select_first();
                    };
                }
                Connect::Info => {
                    let info = Info {
                        name: "".to_string(),
                        description: "".to_string(),
                        language: "".to_string(),
                        tags: Vec::new(),
                    };
                    self.server = Some(Server {
                        info: Some(Arc::new(Mutex::new(info))),
                        messages: Some(Arc::new(Mutex::new(Vec::<String>::new()))),
                        ip: Some(self.msg_buffer.clone()),
                    });
                    if let Some(server) = &self.server {
                        let server_info = Arc::clone(server.info.as_ref().unwrap());
                        let ip = server.ip.clone().unwrap();
                        tokio::spawn(async move { make_info_request(server_info, ip).await });
                    }
                }
            },
            Scene::Connecting => {
                self.list = None;
                self.list_state = None;
                if !self.server.is_some() {
                    let info = Info {
                        name: "".to_string(),
                        description: "".to_string(),
                        language: "".to_string(),
                        tags: Vec::new(),
                    };
                    self.server = Some(Server {
                        info: Some(Arc::new(Mutex::new(info))),
                        messages: Some(Arc::new(Mutex::new(Vec::<String>::new()))),
                        ip: Some(self.msg_buffer.clone()),
                    });
                }
                self.connection_state = Some(Arc::new(Mutex::new(Connection::Connecting)));
            }
            Scene::Message => {
                self.list = None;
                self.list_state = None;

                if let Some(server) = self.server.as_mut() {
                    server.messages = Some(Arc::new(Mutex::new(Vec::new())));
                }
            }
        }
    }
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
