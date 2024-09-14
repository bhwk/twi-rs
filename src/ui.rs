use crate::{app::App, components};
use ratatui::{
    layout::{Constraint, Layout},
    terminal::Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(4),
    ]);
    let [tabs_area, messages_area, input_area] = vertical.areas(frame.size());

    components::input::render_input_box(app, input_area, frame);
    components::messages::render_messages(app, messages_area, frame);
    components::tabs::render_tabs(app, tabs_area, frame);
}
