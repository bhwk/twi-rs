use std::{collections::HashMap, error::Error};

use crossterm::cursor::position;
use irc::client::Client;
use tokio_util::sync::CancellationToken;

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Default)]
pub struct MessageInfo {
    pub nickname: String,
    pub channel: String,
    pub content: String,
}

pub struct ChannelInfo {
    pub name: String,
    pub messages: Vec<MessageInfo>,
}

impl ChannelInfo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            messages: Vec::new(),
        }
    }
}

pub struct App {
    pub running: bool,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub input: String,
    pub character_index: usize,
    pub channels: Vec<ChannelInfo>,
    irc_client: Client,
    cancel_token: CancellationToken,
}

impl App {
    pub fn new(irc_client: Client, cancel_token: CancellationToken) -> Self {
        Self {
            running: true,
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
            channels: Vec::new(),
            irc_client,
            cancel_token,
        }
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_input_message(&mut self) {
        self.messages.push(self.input.clone());
        self.irc_client
            .send_privmsg("#blanlita", self.input.clone())
            .unwrap();
        self.input.clear();
        self.reset_cursor();
    }

    pub fn push_irc_message(&mut self, chat_message: String) {
        self.messages.push(chat_message);
    }

    pub fn add_chat_message(&mut self, chat_message: MessageInfo) {
        let channel = self
            .channels
            .iter_mut()
            .find(|channel| channel.name == chat_message.channel);
        if let Some(channel) = channel {
            channel.messages.push(chat_message)
        }
    }

    pub fn quit(&mut self) {
        self.cancel_token.cancel();
        self.running = false;
    }
}
