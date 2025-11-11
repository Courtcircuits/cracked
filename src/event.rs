use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::Duration;

use crate::app::App;

pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))?
        && let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        handle_key_event(app, key);
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next_challenge();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous_challenge();
        }
        KeyCode::Char('d') | KeyCode::Enter => {
            // This will trigger the download in main loop
            if app.get_selected_challenge().is_some() {
                app.trigger_download();
            }
        }
        _ => {}
    }
}
