use dotenv::dotenv;
use std::{env, error::Error};
use tokio::sync::mpsc;

mod event;
mod twitch;

use event::Event;
use twitch::irc_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let ouath_token = env::var("OAUTH")?;

    let (tx, mut rx) = mpsc::unbounded_channel();

    let irc_handle = tokio::spawn(async move {
        if let Err(e) = irc_client::irc_client(tx, ouath_token).await {
            eprintln!("Error: {}", e)
        }
    });

    while let Some(event) = rx.recv().await {
        match event {
            Event::IrcEvent(irc_event) => match irc_event {
                irc_client::IrcEvent::Join(channel) => {
                    println!("JOIN [{}]", channel)
                }
                irc_client::IrcEvent::Privmsg(channel, message, nickname_exists) => {
                    if let Some(nickname) = nickname_exists {
                        println!("[{}] {}: {}", channel, nickname, message)
                    } else {
                        println!("[{}] anon: {}", channel, message)
                    }
                }
                irc_client::IrcEvent::Ping(server) => println!("Got ping from: {server}"),
                irc_client::IrcEvent::Other(_) => {}
            },
        }
    }

    irc_handle.await?;

    Ok(())
}
