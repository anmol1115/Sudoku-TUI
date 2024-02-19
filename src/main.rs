use crossterm::{
    event::{self, KeyCode, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Stdout, Result};

mod errors;
mod dataset;
mod app;
mod ui;

use ui::ui;
use app::{App, CurrentScreen};

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Menu => match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Tab => app.toggle_menu_selection(),
                    _ => ()
                },
                _ => ()
            }
        }
    }
}

fn main() -> Result<()> {
    dataset::initialize().expect("");

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut app = App::new();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    run_app(&mut terminal, &mut app)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
