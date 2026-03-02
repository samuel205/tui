mod app;
mod ui;

use crossterm::{
    terminal::{ 
        LeaveAlternateScreen,
        EnterAlternateScreen
    }};


fn main() {
    
    std::panic::set_hook(Box::new(|panic_info| {
        let _ = disable_todo();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        eprintln!("Panic occurred: {:?}", panic_info);
    }));

    execute!(io::stdout(), EnterAlternateScreen);

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()));

    let mut app = app:App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app));

        if event::pool(std::time::Duration::from_millis(100)) {
            if let Event::Key(key) = event::read()?{
                if key.kind == KeyEventKind::Press { continue; }

                match app.mode{
                    Mode::Editing => match key.code {
                        KeyCode::Enter => app.confirm(),
                        KeyCode::Esc => app.cancel(),
                        _ => {}
                    },
                    Mode::Deleting => match key.code {
                        KeyCode::Enter | KeyCode::Char('y) => app.confirm(),
                        KeyCode::Esc => app.cancel(),
                        _ => {}
                    },
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => app.add(),
                        KeyCode::Char('e') => app.edit(),
                        KeyCode::Char('d') => app.delete(),
                        KeyCode::Char('f') => app.filter(),
                        KeyCode::Up => app.up(),
                        KeyCode::Down => app.down(),
                        _ => {}
                    }

                }
            }
        }
    }

}
