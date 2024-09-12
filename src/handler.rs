use crate::{
    app::{App, AppResult, InputMode, MessageInfo},
    twitch::client_stream::IrcEvent,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            // Exit application on `ESC` or `q`
            // only in NORMAL mode
            KeyCode::Esc | KeyCode::Char('q') => {
                app.quit();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            KeyCode::Tab => {
                //TODO!
                todo!()
            }
            // enter edit mode
            KeyCode::Char('e') => app.input_mode = InputMode::Editing,
            _ => {}
        },

        InputMode::Editing => match key_event.code {
            KeyCode::Char(to_insert) => app.enter_char(to_insert),
            KeyCode::Enter => {
                app.submit_input_message();
            }
            KeyCode::Backspace => {
                app.delete_char();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Right => {
                app.move_cursor_right();
            }
            KeyCode::Left => {
                app.move_cursor_left();
            }
            _ => {}
        },
    }
    Ok(())
}

pub fn handle_irc_messages(irc_event: IrcEvent, app: &mut App) -> AppResult<()> {
    match irc_event {
        IrcEvent::Privmsg(channel, msg, nickname) => {
            let chat_msg: String;
            let mut chat_message = MessageInfo::default();
            if let Some(nick) = nickname {
                chat_msg = format!("[{}] {}: {}", channel, nick, msg);
                chat_message.channel = channel;
                chat_message.content = msg;
                chat_message.nickname = nick;
            } else {
                chat_msg = format!("[{}] anon: {}", channel, msg);
                chat_message.channel = channel;
                chat_message.content = msg;
                chat_message.nickname = "UNKNOWN".to_string();
            }
            app.push_irc_message(chat_msg);
            app.add_chat_message(chat_message);
        }
        IrcEvent::Join(channel) => {
            let join_msg = format!("Joined [{}]", channel);
            app.push_irc_message(join_msg);
        }
        _ => {}
    }

    Ok(())
}
