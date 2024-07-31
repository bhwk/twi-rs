use crate::{
    app::{App, AppResult, InputMode},
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
            let mut chat_msg = String::new();
            if let Some(nick) = nickname {
                chat_msg = format!("[{}] {}: {}", channel, nick, msg);
            } else {
                chat_msg = format!("[{}] anon: {}", channel, msg);
            }
            app.push_irc_message(chat_msg);
        }
        IrcEvent::Join(channel) => {
            let join_msg = format!("Joined [{}]", channel);
            app.push_irc_message(join_msg);
        }
        _ => {}
    }

    Ok(())
}
