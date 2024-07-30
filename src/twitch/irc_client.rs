use futures::prelude::*;
use irc::client::prelude::*;
use std::error::Error;
use tokio::sync::mpsc;

use crate::event::Event;

#[derive(Debug)]
pub enum IrcEvent {
    // channel name, message content, source nickname(if it exists)
    Privmsg(String, String, Option<String>),
    Join(String),
    Ping(String),
    Other(Box<Message>),
}

impl From<Message> for IrcEvent {
    fn from(message: Message) -> Self {
        match message.command {
            Command::PRIVMSG(ref channel, ref msg) => {
                if let Some(nickname) = message.source_nickname() {
                    IrcEvent::Privmsg(channel.clone(), msg.clone(), Some(nickname.to_string()))
                } else {
                    IrcEvent::Privmsg(channel.clone(), msg.clone(), None)
                }
            }
            Command::JOIN(channel, _, _) => IrcEvent::Join(channel),
            Command::PING(server, _) => IrcEvent::Ping(server),
            _ => IrcEvent::Other(Box::new(message)),
        }
    }
}

pub async fn irc_client(
    tx: mpsc::UnboundedSender<Event>,
    ouath_token: String,
) -> Result<(), Box<dyn Error>> {
    let config = Config {
        nickname: Some("blanlita".to_owned()),
        password: Some(ouath_token.to_owned()),
        server: Some("irc.chat.twitch.tv".to_owned()),
        channels: vec!["#roflgator".into()],
        ..Config::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        if let Err(e) = tx.send(Event::IrcEvent(message.into())) {
            return Err(Box::new(e));
        }
    }

    Ok(())
}
