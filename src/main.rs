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
use app::{App, CurrentScreen, MenuSelection};

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
                    KeyCode::Enter => match app.menu_selection {
                        MenuSelection::ChooseDifficulty => {
                            app.current_screen = CurrentScreen::DifficultySelect
                        },
                        MenuSelection::RandomDifficulty => {
                            app.set_pair(None, true)?;
                        }
                    }
                    _ => ()
                },
                CurrentScreen::DifficultySelect => match key.code {
                    KeyCode::Esc => app.current_screen = CurrentScreen::Menu,
                    KeyCode::Up => app.decrement_difficulty(),
                    KeyCode::Down => app.increment_difficulty(),
                    KeyCode::Enter => app.select_difficulty()?,
                    _ => ()
                },
                CurrentScreen::Playground => match key.code {
                    KeyCode::Esc => app.current_screen = CurrentScreen::Menu,
                    KeyCode::Up => app.move_cursor(-1, 0),
                    KeyCode::Down => app.move_cursor(1, 0),
                    KeyCode::Left => app.move_cursor(0, -1),
                    KeyCode::Right => app.move_cursor(0, 1),
                    KeyCode::Char(character) => match character {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => app.set_value(character),
                        'v' => app.validate(),
                        'r' => app.reset(),
                        _ => ()
                    }
                    _ => ()
                }
                _ => ()
            }
        }
    }
}

fn main() -> Result<()> {
    dataset::initialize().expect("");

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut app = App::new()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    run_app(&mut terminal, &mut app)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
