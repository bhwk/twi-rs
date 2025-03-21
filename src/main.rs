use dotenv::dotenv;
use event::{Event, EventHandler};

use irc_handler::handle_irc_messages;
use key_handler::handle_key_events;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio_util::sync::CancellationToken;
use tui::Tui;

mod app;
mod components;
mod event;
mod irc_handler;
mod join_input;
mod key_handler;
mod messagebox;
mod tui;
mod twitch;
mod ui;

use crate::app::{App, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();

    let cancel_token = CancellationToken::new();

    //clone cancel token to pass to events handler
    let cloned_cancel_token = cancel_token.clone();

    // create irc client and stream
    let (client, client_stream) = twitch::client_stream::create_client_stream().await?;
    let mut app = App::new(client, cancel_token);

    // init terminal ui
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(client_stream, cloned_cancel_token);

    let mut tui = Tui::new(terminal, events);

    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        if let Some(event) = tui.events.next().await {
            match event {
                Event::Client(irc_event) => handle_irc_messages(irc_event, &mut app)?,
                Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                Event::Resize => tui.resize()?,
            }
        }
    }

    tui.exit()?;
    Ok(())
}
