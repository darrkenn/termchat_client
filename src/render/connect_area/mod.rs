use ratatui::{
    Frame,
    layout::{Constraint, Rect},
};

use crate::{
    app::{App, Connect},
    render::{
        connect_area::{info::render_info, menu::render_menu},
        render::center,
    },
};

pub mod info;
pub mod menu;

pub fn render_connect_area(frame: &mut Frame, area: Rect, app: &mut App, connect_scene: Connect) {
    let area = center(area, Constraint::Percentage(30), Constraint::Percentage(50));
    match connect_scene {
        Connect::Info => {
            render_info(frame, area, app);
        }
        Connect::Menu => {
            render_menu(frame, area, app);
        }
    }
}
