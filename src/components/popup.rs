use ratatui::{
    layout::Rect,
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_popup(frame: &mut Frame, area: Rect) {
    let popup = Paragraph::new("Popup content").block(Block::bordered().title("Popup"));
    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}
