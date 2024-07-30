use dotenv::dotenv;
use futures::{FutureExt, StreamExt};
use handler::EventHandler;
use std::{env, error::Error};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crossterm::event as CrosstermEvent;

mod event;
mod handler;
mod twitch;

use event::Event;
use twitch::client_stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let oauth_token = env::var("OAUTH")?;

    let mut event_handler = EventHandler::new(oauth_token, 250);

    while let Some(event) = event_handler.next().await {
        match event {
            Event::IrcEvent(irc_event) => match irc_event {
                client_stream::IrcEvent::Join(channel) => {
                    println!("JOIN [{}]", channel)
                }
                client_stream::IrcEvent::Privmsg(channel, message, nickname_exists) => {
                    if let Some(nickname) = nickname_exists {
                        println!("[{}] {}: {}", channel, nickname, message)
                    } else {
                        println!("[{}] anon: {}", channel, message)
                    }
                }
                client_stream::IrcEvent::Ping(server) => println!("Got ping from: {server}"),
                client_stream::IrcEvent::Other(_) => {}
            },
            Event::Quit => {
                println!("Received Quit");
                break;
            }
        }
    }

    Ok(())
}
