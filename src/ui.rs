use crate::{
    app::{App, AppMode},
    components,
};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(4),
    ]);

    let [tabs_area, message_box_area, input_area] = vertical.areas(frame.area());

    match app.app_mode {
        AppMode::Normal => {
            components::input::render_input_box(app, input_area, frame);
            components::messages::render_messages(app, message_box_area, frame);
            components::tabs::render_tabs(app, tabs_area, frame);
        }

        AppMode::Joining => components::popup::render_popup(
            frame,
            center(
                frame.area(),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
            ),
        ),
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
