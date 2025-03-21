use crate::twitch::client_stream;
use crossterm::event::KeyEvent;
use futures::{FutureExt, StreamExt};
use irc::client::ClientStream;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub enum Event {
    Client(client_stream::ClientEvent),
    Key(KeyEvent),
    Resize,
}

#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
    handler: tokio::task::JoinHandle<()>,
}

// create all our handlers here,
// irc handler for twitch to terminal events
// key input reading from crossterm for key handling
impl EventHandler {
    pub fn new(mut stream: ClientStream, cloned_cancel_token: CancellationToken) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();

        let irc_cancel_token = cloned_cancel_token.clone();
        let irc_sender = sender.clone();

        // handle irc twitch events
        let _irc_handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = irc_cancel_token.cancelled() => {
                        break;
                    }
                    message = stream.next() => {
                        if let Some(Ok(message)) = message {
                        irc_sender.send(Event::Client(message.into())).unwrap();
                    };
                    }
                }
            }
        });

        // handle key presses
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            loop {
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                _ = cloned_cancel_token.cancelled() => {
                break;
                }

                Some(Ok(evt)) = crossterm_event => {
                    match evt {
                        crossterm::event::Event::Key(key) => _sender.send(Event::Key(key)).unwrap(),
                        crossterm::event::Event::Resize(_, _) => _sender.send(Event::Resize).unwrap(),
                        _ => {}
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

    // get next event from receiver
    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }
}
