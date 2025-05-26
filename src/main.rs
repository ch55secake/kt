mod app;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{error::Error, io};

use crate::app::{App, Page};
use crate::ui::draw;

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

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    if app.command_mode {
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
                            KeyCode::Char(c) => {
                                app.command_input.push(c);
                            }
                            KeyCode::Backspace => {
                                app.command_input.pop();
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
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
                }
                _ => {}
            }
        }
    }
}
