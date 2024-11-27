use crate::{
    app::{App, AppMode, AppResult},
    messagebox::MessageMode,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.app_mode {
        AppMode::Normal => {
            match app.message_box.mode {
                MessageMode::Normal => match key_event.code {
                    // Exit application on `Ctrl-c` or `Ctrl-q`
                    KeyCode::Char('q') | KeyCode::Char('c') => {
                        if key_event.modifiers == KeyModifiers::CONTROL {
                            app.quit();
                        }
                    }
                    KeyCode::Char('x') => {
                        app.leave_current_channel();
                    }
                    KeyCode::Char('\\') => app.app_mode = AppMode::Joining,
                    KeyCode::Char('j') | KeyCode::Down => {
                        let offset = app.list_state.offset();
                        *app.list_state.offset_mut() = offset.saturating_sub(1);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        let offset = app.list_state.offset();
                        *app.list_state.offset_mut() = offset.saturating_add(1);
                    }
                    KeyCode::Tab => app.next_channel(),
                    // enter edit mode
                    KeyCode::Char('i') => app.message_box.mode = MessageMode::Editing,
                    _ => {}
                },

                MessageMode::Editing => match key_event.code {
                    KeyCode::Char(to_insert) => app.message_box.enter_char(to_insert),
                    KeyCode::Enter => {
                        app.send_chat_message();
                    }
                    KeyCode::Backspace => {
                        app.message_box.delete_char();
                    }
                    KeyCode::Esc => {
                        app.message_box.mode = MessageMode::Normal;
                    }
                    KeyCode::Right => {
                        app.message_box.move_cursor_right();
                    }
                    KeyCode::Left => {
                        app.message_box.move_cursor_left();
                    }
                    _ => {}
                },
            }
        }

        AppMode::Joining => match key_event.code {
            KeyCode::Esc => {
                app.app_mode = AppMode::Normal;
            }
            KeyCode::Enter => {
                app.join_channel();
            }
            KeyCode::Char(to_insert) => app.join_box.enter_char(to_insert),
            KeyCode::Backspace => {
                app.join_box.delete_char();
            }
            KeyCode::Right => {
                app.join_box.move_cursor_right();
            }
            KeyCode::Left => {
                app.join_box.move_cursor_left();
            }
            _ => {}
        },
    }
    Ok(())
}
