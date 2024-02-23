use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result, Stdout};

mod app;
mod dataset;
mod errors;
mod solver;
mod ui;

use app::{App, CurrentScreen, MenuSelection};
use ui::ui;

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Menu => match key.code {
                    KeyCode::Esc => app.current_screen = CurrentScreen::Exiting,
                    KeyCode::Tab => app.toggle_menu_selection(),
                    KeyCode::Enter => match app.menu_selection {
                        MenuSelection::ChooseDifficulty => {
                            app.current_screen = CurrentScreen::DifficultySelect
                        }
                        MenuSelection::RandomDifficulty => {
                            app.set_pair(None, true)?;
                        }
                    },
                    _ => (),
                },
                CurrentScreen::DifficultySelect => match key.code {
                    KeyCode::Esc => app.current_screen = CurrentScreen::Menu,
                    KeyCode::Up => app.decrement_difficulty(),
                    KeyCode::Down => app.increment_difficulty(),
                    KeyCode::Enter => app.select_difficulty()?,
                    _ => (),
                },
                CurrentScreen::Playground => match key.code {
                    KeyCode::Esc => app.current_screen = CurrentScreen::Menu,
                    KeyCode::Up => app.move_cursor(-1, 0),
                    KeyCode::Down => app.move_cursor(1, 0),
                    KeyCode::Left => app.move_cursor(0, -1),
                    KeyCode::Right => app.move_cursor(0, 1),
                    KeyCode::Char(character) => match character {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            app.set_value(character)
                        }
                        'v' => app.validate(),
                        'r' => app.reset(),
                        's' => app.current_screen = CurrentScreen::SolverSelect,
                        _ => (),
                    },
                    _ => (),
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => return Ok(()),
                    KeyCode::Char('n') => app.current_screen = CurrentScreen::Menu,
                    _ => (),
                },
                CurrentScreen::SolverSelect => match key.code {
                    KeyCode::Esc => app.current_screen = CurrentScreen::Playground,
                    KeyCode::Down => app.increment_solver(),
                    KeyCode::Up => app.decrement_solver(),
                    KeyCode::Enter => app.current_screen = CurrentScreen::Solver,
                    _ => (),
                },
                CurrentScreen::Solver => match key.code {
                    KeyCode::Esc => app.current_screen = CurrentScreen::Playground,
                    _ => (),
                },
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
