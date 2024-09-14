use ratatui::{
    layout::Rect,
    text::{self, Line},
    widgets::{Block, List, ListItem},
    Frame,
};

use crate::app::{App, MessageInfo};

pub fn render_messages(app: &App, area: Rect, frame: &mut Frame) {
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
        let messages: Vec<ListItem> = wrapped_text
            .iter()
            .map(|s| {
                let line = Line::from(s.to_string());
                ListItem::new(line)
            })
            .collect();

        let messages = List::new(messages).block(Block::bordered().title("messages"));
        frame.render_widget(messages, area);
    } else {
        let messages: Vec<ListItem> = vec![];
        let messages = List::new(messages).block(Block::bordered().title("messages"));
        frame.render_widget(messages, area);
    }
}
