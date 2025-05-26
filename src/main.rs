mod app;
mod ui;

use crate::app::{App, Page};
use crate::ui::draw;
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| draw::<B>(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if app.command_mode {
                    handle_command_palette_input(&mut app, key);
                } else {
                    handle_key_nav(&mut app, key);
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
            if let Event::Mouse(mouse) = event::read()? {
                handle_mouse_nav(&mut app, mouse);
            }
        }
    }
}

/// Handle scrolling up and down through the list
fn handle_mouse_nav(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollDown => app.move_down(),
        MouseEventKind::ScrollUp => app.move_up(),
        _ => {}
    }
}

/// Handle ordinary navigation, moving up and down the page
fn handle_key_nav(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('?') => {
            app.show_help = !app.show_help;
        }
        KeyCode::Right => {
            app.current_page = app.current_page.next();
            app.reset_selection();
        }
        KeyCode::Left => {
            app.current_page = app.current_page.previous();
            app.reset_selection();
        }
        KeyCode::Up => app.move_up(),
        KeyCode::Down => app.move_down(),
        KeyCode::Char('/') => {
            app.command_mode = true;
        }
        _ => {}
    }
}

/// Handle the input to the command palette
fn handle_command_palette_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.command_mode = false;
            app.command_input.clear();
        }
        KeyCode::Enter => {
            let input = app.command_input.trim();
            if let Some(page) = Page::from_str(input) {
                app.current_page = page;
                app.reset_selection();
            }
            app.command_mode = false;
            app.command_input.clear();
        }
        KeyCode::Char(char) => {
            app.command_input.push(char);
        }
        KeyCode::Backspace => {
            app.command_input.pop();
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crossterm::event::KeyModifiers;

    #[test]
    fn test_handle_command_palette_input() {
        let mut app = App::new();

        handle_command_palette_input(&mut app, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

        assert_eq!(app.command_mode, false);
        assert!(app.command_input.is_empty());

        handle_command_palette_input(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert_eq!(app.command_mode, false);
        assert!(app.command_input.is_empty());

        app.command_input = "banana".to_string();
        handle_command_palette_input(
            &mut app,
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
        );

        assert!(app.command_input.contains("banan"));
    }

    #[test]
    fn test_handle_mouse_nav() {}

    #[test]
    fn test_handle_key_nav() {}
}
