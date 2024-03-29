use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

mod difficulty_select;
mod exiting;
mod menu;
mod playground;
mod solver;
mod solver_select;
mod util;
use crate::app;

pub fn ui(frame: &mut Frame, app: &mut app::App) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(9),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Span::styled(
        "Sudoku in CLI",
        Style::default().fg(Color::White),
    ))
    .block(title_block);

    frame.render_widget(title, sections[0]);

    match app.current_screen {
        app::CurrentScreen::Menu => menu::display(frame, sections[1], sections[2], app),
        app::CurrentScreen::DifficultySelect => {
            difficulty_select::display(frame, sections[1], sections[2], app)
        }
        app::CurrentScreen::Playground => playground::display(frame, sections[1], sections[2], app),
        app::CurrentScreen::Exiting => exiting::display(frame, sections[1], sections[2]),
        app::CurrentScreen::SolverSelect => {
            solver_select::display(frame, sections[1], sections[2], app)
        }
        app::CurrentScreen::Solver => solver::display(frame, sections[1], sections[2], app),
    };
}
