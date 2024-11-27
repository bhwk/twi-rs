use irc::client::Client;
use ratatui::widgets::ListState;
use std::error::Error;
use tokio_util::sync::CancellationToken;

use crate::{join_input::JoinBox, messagebox::MessageBox};

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

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

#[derive(Default)]
pub enum AppMode {
    #[default]
    Normal,
    Joining,
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
    pub message_box: MessageBox,
    pub join_box: JoinBox,
    pub channels: Vec<ChannelInfo>,
    pub current_channel: usize,
    pub list_state: ListState,
    pub app_mode: AppMode,
    client: Client,
    cancel_token: CancellationToken,
}

impl App {
    pub fn new(client: Client, cancel_token: CancellationToken) -> Self {
        Self {
            running: true,
            message_box: MessageBox::default(),
            join_box: JoinBox::default(),
            channels: Vec::new(),
            current_channel: 0,
            list_state: ListState::default(),
            app_mode: AppMode::default(),
            client,
            cancel_token,
        }
    }

    pub fn send_chat_message(&mut self) {
        if self.message_box.input.is_empty() {
            return;
        }

        let current_channel = self.channels.get_mut(self.current_channel);
        if let Some(channel) = current_channel {
            self.client
                .send_privmsg(channel.name.clone(), self.message_box.input.clone())
                .unwrap();
            channel.messages.push(MessageInfo {
                nickname: self.client.current_nickname().into(),
                content: self.message_box.input.clone(),
            });
            self.message_box.input.clear();
            self.message_box.reset_cursor();
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

    pub fn join_channel(&mut self) {
        if !self.join_box.channel.starts_with("#") {
            self.join_box.channel = format!("#{}", self.join_box.channel)
        }

        self.client
            .send_join(self.join_box.channel.clone())
            .unwrap_or_default();

        self.join_box.channel.clear();
        self.join_box.reset_cursor();
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
            self.client.send_part(channel.name.clone()).unwrap();
            self.channels.remove(self.current_channel);
            self.current_channel = self.current_channel.saturating_sub(1);
        }
    }

    pub fn quit(&mut self) {
        self.cancel_token.cancel();
        self.running = false;
    }
}
