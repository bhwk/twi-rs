use crate::{
    app::{App, AppResult, MessageInfo},
    twitch::client_stream::ClientEvent,
};

pub fn handle_irc_messages(irc_event: ClientEvent, app: &mut App) -> AppResult<()> {
    match irc_event {
        ClientEvent::Privmsg(channel, msg, nickname) => {
            let mut chat_message = MessageInfo::default();
            if let Some(nick) = nickname {
                chat_message.content = msg;
                chat_message.nickname = nick;
            } else {
                chat_message.content = msg;
                chat_message.nickname = "UNKNOWN".to_string();
            }
            app.add_chat_message(channel, chat_message);
        }
        ClientEvent::Join(channel) => {
            app.on_join_channel(channel);
        }
        _ => {}
    }

    Ok(())
}
