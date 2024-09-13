use crate::{
    app::{App, AppResult, InputMode, MessageInfo},
    twitch::client_stream::IrcEvent,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            // Exit application on or `q`
            // only in NORMAL mode
            KeyCode::Char('q') => {
                app.quit();
            }
            KeyCode::Char('x') => {
                app.leave_current_channel();
            }
            // Exit application on `Ctrl-c`
            KeyCode::Char('c') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            KeyCode::Tab => app.next_channel(),
            // enter edit mode
            KeyCode::Char('e') => app.input_mode = InputMode::Editing,
            _ => {}
        },

        InputMode::Editing => match key_event.code {
            KeyCode::Char(to_insert) => app.enter_char(to_insert),
            KeyCode::Enter => {
                app.send_chat_message();
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
        IrcEvent::Join(channel) => {
            app.on_join_channel(channel);
        }
        _ => {}
    }

    Ok(())
}
