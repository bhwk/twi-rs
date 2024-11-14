use irc::client::Client;
use ratatui::widgets::ListState;
use std::error::Error;
use tokio_util::sync::CancellationToken;

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Default, PartialEq, Eq)]
pub struct MessageInfo {
    pub nickname: String,
    pub content: String,
}

#[derive(PartialEq, Eq)]
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
    pub input: String,
    pub character_index: usize,
    pub channels: Vec<ChannelInfo>,
    pub current_channel: usize,
    pub state: ListState,
    irc_client: Client,
    cancel_token: CancellationToken,
}

impl App {
    pub fn new(irc_client: Client, cancel_token: CancellationToken) -> Self {
        Self {
            running: true,
            input: String::new(),
            input_mode: InputMode::Normal,
            character_index: 0,
            channels: Vec::new(),
            current_channel: 0,
            state: ListState::default(),
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

    pub fn send_chat_message(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let current_channel = self.channels.get_mut(self.current_channel);
        if let Some(channel) = current_channel {
            self.irc_client
                .send_privmsg(channel.name.clone(), self.input.clone())
                .unwrap();
            channel.messages.push(MessageInfo {
                nickname: self.irc_client.current_nickname().into(),
                content: self.input.clone(),
            });
            self.input.clear();
            self.reset_cursor();
        }
    }

    pub fn add_chat_message(&mut self, target_channel: String, chat_message: MessageInfo) {
        let channel = self
            .channels
            .iter_mut()
            .find(|channel| channel.name == target_channel);
        // if channel doesn't exist we just die
        if let Some(channel) = channel {
            channel.messages.push(chat_message)
        }
    }

    pub fn join_channel(&mut self, chanlist: Vec<String>) {
        for channel in chanlist {
            self.irc_client.send_join(channel).unwrap();
        }
    }

    pub fn on_join_channel(&mut self, channel: String) {
        if self.channels.iter_mut().any(|c| c.name == channel) {
        } else {
            self.channels.push(ChannelInfo::new(channel));
        }
    }

    pub fn next_channel(&mut self) {
        if self.channels.is_empty() {
            return;
        }
        if self.current_channel >= self.channels.len() - 1 && !self.channels.is_empty() {
            self.current_channel = 0;
        } else {
            self.current_channel += 1;
        }
    }

    pub fn leave_current_channel(&mut self) {
        if let Some(channel) = self.channels.get(self.current_channel) {
            self.irc_client.send_part(channel.name.clone()).unwrap();
            self.channels.remove(self.current_channel);
            self.current_channel = self.current_channel.saturating_sub(1);
        }
    }

    pub fn quit(&mut self) {
        self.cancel_token.cancel();
        self.running = false;
    }
}
