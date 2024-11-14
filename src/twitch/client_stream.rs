use irc::client::{prelude::*, ClientStream};

use crate::app::AppResult;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClientEvent {
    // channel name, message content, source nickname(if it exists)
    Privmsg(String, String, Option<String>),
    // Channel name
    Join(String),
    Leave(String),
    // Channel Name
    Ping(String),
    Other(Box<Message>),
}

impl From<Message> for ClientEvent {
    fn from(message: Message) -> Self {
        match message.command {
            Command::PRIVMSG(ref channel, ref msg) => {
                if let Some(nickname) = message.source_nickname() {
                    ClientEvent::Privmsg(channel.clone(), msg.clone(), Some(nickname.to_string()))
                } else {
                    ClientEvent::Privmsg(channel.clone(), msg.clone(), None)
                }
            }
            Command::JOIN(channel, _, _) => ClientEvent::Join(channel),
            Command::PART(channel, _) => ClientEvent::Leave(channel),
            Command::PING(server, _) => ClientEvent::Ping(server),
            _ => ClientEvent::Other(Box::new(message)),
        }
    }
}

pub async fn create_client_stream() -> AppResult<(Client, ClientStream)> {
    let config = Config::load("config.toml").unwrap();

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let stream = client.stream()?;

    Ok((client, stream))
}
