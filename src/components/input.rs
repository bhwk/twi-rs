use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{app::App, messagebox::MessageMode};

pub fn render_message_box(app: &App, area: Rect, frame: &mut Frame) {
    use Constraint::Length;

    let layout = Layout::vertical([Length(1), Length(3)]);
    let [help_area, message_box] = layout.areas(area);
    let (msg, style) = match app.message_box.mode {
        MessageMode::Normal => (
            vec!["Press <ctrl + q> to exit, <i> to edit".into()],
            Style::default(),
        ),
        MessageMode::Editing => (
            vec!["Press ESC to return to normal mode".into()],
            Style::default().add_modifier(Modifier::DIM),
        ),
    };
    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);

    let input = Paragraph::new(app.message_box.input.as_str())
        .style(match app.message_box.mode {
            MessageMode::Normal => Style::default(),
            MessageMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"));
    match app.message_box.mode {
        MessageMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        MessageMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor_position(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                (
                    message_box.x + app.message_box.character_index as u16 + 1,
                    // Move one line down, from the border to the input line
                    message_box.y + 1,
                ),
            );
        }
    }
    frame.render_widget(help_message, help_area);
    frame.render_widget(input, message_box);
}
