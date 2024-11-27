use crate::app::{App, AppMode, AppResult, InputMode};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.app_mode {
        AppMode::Normal => {
            match app.input_mode {
                InputMode::Normal => match key_event.code {
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
                    KeyCode::Char('i') => app.input_mode = InputMode::Editing,
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
        }

        AppMode::Joining => match key_event.code {
            KeyCode::Char('q') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.app_mode = AppMode::Normal
                }
            }
            _ => {}
        },
    }
    Ok(())
}
