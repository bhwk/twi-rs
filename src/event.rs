use crate::twitch::client_stream;

pub enum Event {
    IrcEvent(client_stream::IrcEvent),
    Quit,
}
