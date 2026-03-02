mod app;
mod ui;

use std::{error::Error, io, time::Duration};

use app::Mode;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut app = app::App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match app.mode {
                    Mode::Editing => match key.code {
                        KeyCode::Enter => app.confirm(),
                        KeyCode::Esc => app.cancel(),
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Char(ch) => app.input.push(ch),
                        _ => {}
                    },
                    Mode::Deleting => match key.code {
                        KeyCode::Enter | KeyCode::Char('y') => app.confirm(),
                        KeyCode::Esc => app.cancel(),
                        _ => {}
                    },
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => app.add(),
                        KeyCode::Char('e') => app.edit(),
                        KeyCode::Char('d') => app.delete(),
                        KeyCode::Char('f') => app.filter(),
                        KeyCode::Char(' ') => app.confirm(),
                        KeyCode::Up => app.up(),
                        KeyCode::Down => app.down(),
                        _ => {}
                    },
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
