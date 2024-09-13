use crate::app::{App, InputMode, MessageInfo};
use ratatui::{
    layout::{Constraint, Layout},
    style::{palette::tailwind, Color, Modifier, Style},
    terminal::Frame,
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph, Tabs},
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
        Constraint::Length(3),
    ]);
    let [tabs_area, messages_area, help_area, input_area] = vertical.areas(frame.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec!["Press (q) to exit, (e) to edit".into()],
            Style::default(),
        ),
        InputMode::Editing => (
            vec!["Press ESC to return to normal mode".into()],
            Style::default().add_modifier(Modifier::DIM),
        ),
    };
    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"));
    frame.render_widget(input, input_area);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + app.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            );
        }
    }

    let current_channel = app.channels.get(app.current_channel);
    if let Some(channel) = current_channel {
        let messages: Vec<ListItem> = channel
            .messages
            .iter()
            .map(|message_info: &MessageInfo| {
                let MessageInfo { nickname, content } = message_info;
                let message_content = Line::from(Span::raw(format!("{nickname}: {content}")));
                ListItem::new(message_content)
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, messages_area);
    } else {
        let messages: Vec<ListItem> = vec![];
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, messages_area);
    }

    // rendering tabs
    let titles = app.channels.iter().map(|f| f.name.clone());
    let selected_tab_index = app.current_channel;
    let tabs = Tabs::new(titles)
        .select(selected_tab_index)
        .highlight_style(tailwind::BLUE.c700)
        .padding("", "")
        .divider(" ");
    frame.render_widget(tabs, tabs_area);
}
