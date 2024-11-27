use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_join_box(app: &App, area: Rect, frame: &mut Frame) {
    use Constraint::Length;

    let layout = Layout::vertical([Length(1), Length(3)]);
    let [help_area, input_box] = layout.areas(area);
    let (msg, style) = (
        vec!["Press <ESC> to return to chat interface".into()],
        Style::default(),
    );
    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);

    let input = Paragraph::new(app.join_box.channel.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::bordered().title("Join Channel"));
    frame.set_cursor_position((
        input_box.x + app.join_box.character_index as u16 + 1,
        input_box.y + 1,
    ));
    frame.render_widget(help_message, help_area);
    frame.render_widget(input, input_box);
}
