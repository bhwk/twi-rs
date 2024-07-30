use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::event::Event;
use crate::twitch::client_stream;

pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(oauth_token: String, tick_rate: u64) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let cancel_token = CancellationToken::new();
        let _sender = sender.clone();

        let irc_cancel_token = cancel_token.clone();
        let irc_sender = sender.clone();

        let irc_handle = tokio::spawn(async move {
            if let Err(e) =
                client_stream::create_client_stream(irc_sender, oauth_token, irc_cancel_token).await
            {
                eprintln!("Error: {}", e)
            }
        });

        let handler = tokio::spawn(async move {
            let tick_rate = std::time::Duration::from_millis(250);
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);

            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = tick_delay => {

                    }

                    Some(Ok(evt)) = crossterm_event => {
                            if let crossterm::event::Event::Key(key) = evt {
                                if key.code  == crossterm::event::KeyCode::Char('q') {
                                    cancel_token.cancel();
                                    _sender.send(Event::Quit).unwrap();
                                    break;
                                }
                            }
                    }

                }
            }
        });

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
}
