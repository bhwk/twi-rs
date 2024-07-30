use crate::twitch::irc_client;

pub enum Event {
    IrcEvent(irc_client::IrcEvent),
}
