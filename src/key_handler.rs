use crate::app::{App, AppResult, InputMode};
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
            KeyCode::Char('j') | KeyCode::Down => {
                let offset = app.state.offset();
                *app.state.offset_mut() = offset.saturating_sub(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let offset = app.state.offset();
                *app.state.offset_mut() = offset.saturating_add(1);
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
