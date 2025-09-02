use std::sync::{Arc, Mutex};

use ratatui::widgets::{ListItem, ListState};

pub enum Connection {
    Connected,
    Error(String),
    Close,
}

pub enum Scene {
    Menu,
    Message,
    Settings,
}

pub struct App<'a> {
    pub scene: Scene,
    pub list_state: Option<ListState>,
    pub list: Option<Vec<ListItem<'a>>>,
    pub connection_state: Option<Connection>,
    pub msg_buffer: String,
    pub messages: Option<Arc<Mutex<Vec<String>>>>,
}

impl App<'_> {
    pub fn update_list(&mut self) {
        match self.scene {
            Scene::Menu => {
                self.list = Some(vec![
                    ListItem::new("Connect"),
                    ListItem::new("Saved"),
                    ListItem::new("Settings"),
                ]);
                if let Some(list_state) = self.list_state.as_mut() {
                    list_state.select_first();
                };
                self.messages = None;
            }
            Scene::Settings => {
                self.list = Some(vec![ListItem::new("Manage Accounts")]);
                if let Some(list_state) = self.list_state.as_mut() {
                    list_state.select_first();
                };
            }
            Scene::Message => {
                self.list = None;
                if let Some(list_state) = self.list_state.as_mut() {
                    list_state.select_first();
                }
                self.messages = Some(Arc::new(Mutex::new(Vec::new())));
            }
        }

        let list = vec![
            ListItem::new("Connect"),
            ListItem::new("Saved"),
            ListItem::new("Settings"),
        ];
        self.list = Some(list);
    }
}
