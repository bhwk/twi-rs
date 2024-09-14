use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::Tabs,
    Frame,
};

use crate::app::App;

pub fn render_tabs(app: &App, area: Rect, frame: &mut Frame) {
    // rendering tabs
    let titles = app.channels.iter().map(|f| f.name.clone());
    let selected_tab_index = app.current_channel;
    let tabs = Tabs::new(titles)
        .select(selected_tab_index)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED)
                .add_modifier(Modifier::ITALIC),
        )
        .padding("", "")
        .divider(" ");
    frame.render_widget(tabs, area);
}
