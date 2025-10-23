use ratatui::crossterm::event::{self, KeyEvent, KeyEventKind};

use crate::{
    app::{App, Connection, Scene},
    keyhandling::{
        connect::handle_connect_key, connecting::handle_connecting_key, menu::handle_menu_key,
        message::handle_message_key, saved::handle_saved_key, settings::handle_settings_key,
    },
};

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    if key.code == event::KeyCode::Esc {
        return true;
    }

    let scene = app.scene.clone();
    match scene {
        Scene::Menu => handle_menu_key(key, app),
        Scene::Saved => handle_saved_key(key, app),
        Scene::Connect(connect_scene) => handle_connect_key(key, app, connect_scene),
        Scene::Connecting => handle_connecting_key(key, app),
        Scene::Message => {
            let connection_state = app
                .connection_state
                .as_ref()
                .map(|cs| cs.lock().unwrap().clone());
            if let Some(connection_state) = connection_state {
                match connection_state {
                    Connection::Connected => handle_message_key(key, app),
                    Connection::Close => {
                        if key.code == event::KeyCode::Enter {
                            app.scene = Scene::Menu;
                            app.msg_buffer.clear();
                            app.update_scene();
                        }
                    }
                    _ => {}
                }
            }
        }
        Scene::Settings => handle_settings_key(key),
    }

    false
}
