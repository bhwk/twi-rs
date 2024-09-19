use ratatui::{
    layout::{Margin, Rect},
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::app::{App, MessageInfo};

pub fn render_messages(app: &mut App, area: Rect, frame: &mut Frame) {
    let current_channel = app.channels.get(app.current_channel);
    if let Some(channel) = current_channel {
        let message_contents: Vec<String> = channel
            .messages
            .iter()
            .map(|message_info: &MessageInfo| {
                let MessageInfo { nickname, content } = message_info;
                format!("{nickname}: {content}")
            })
            .collect();
        let wrapped_text: Vec<String> = message_contents
            .iter()
            .flat_map(|s| {
                let mut wrap = textwrap::wrap(s, area.width as usize);
                wrap.iter_mut()
                    .map(|s| s.clone().into_owned())
                    .collect::<Vec<String>>()
            })
            .collect();
        let messages: Vec<Line> = wrapped_text
            .iter()
            .map(|s| Line::from(s.to_string()))
            .collect();

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        app.scrollbar_state = app.scrollbar_state.content_length(messages.len());

        let message_block = Paragraph::new(messages.clone())
            .scroll((app.scroll_position as u16, 0))
            .block(Block::bordered().title("messages"));

        frame.render_widget(message_block, area);
        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut app.scrollbar_state,
        );
    } else {
        let messages: Vec<ListItem> = vec![];
        let messages = List::new(messages).block(Block::bordered().title("messages"));
        frame.render_widget(messages, area);
    }
}
