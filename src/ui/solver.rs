use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Cell, Row, Table},
    Frame,
};

use super::util;
use crate::{app, solver};

pub fn display(frame: &mut Frame, body_section: Rect, footer_section: Rect, app: &mut app::App) {
    body_display(frame, body_section, app);
    footer_display(frame, footer_section);
}

fn body_display(frame: &mut Frame, body_section: Rect, app: &mut app::App) {
    let body_section = util::centered_rect(70, 60, body_section);

    let widths = vec![Constraint::Length(10); 9];
    let mut rows = Vec::new();

    let valid_style = Style::default().fg(Color::Green);
    let existing_style = Style::default().fg(Color::Yellow);

    app.reset();
    match app.solvers.get(app.selected_solver) {
        Some(solver_variant) => match solver_variant {
            app::CurrentSolver::Backtrack(_) => solver::backtrack::solve(&mut app.problem),
            app::CurrentSolver::Constraintbacktrack(_) => solver::constraint_backtrack::solve(&mut app.problem)
        },
        None => false,
    };

    for r in 0..9 {
        let mut column = Vec::new();
        for c in 0..9 {
            let mut entry = Cell::from(app.problem[r][c].to_string().clone());
            if app.valid_entries.contains(&(r, c)) {
                entry = entry.clone().style(valid_style);
            }

            if !app.playable_pos.contains(&(r, c)) {
                entry = entry.clone().style(existing_style);
            }

            column.push(entry);
        }
        rows.push(Row::new(column).height(2));
    }

    let grid = Table::new(rows, widths);

    frame.render_widget(grid, body_section);
}

fn footer_display(frame: &mut Frame, footer_section: Rect) {
    util::footer_display(frame, footer_section, "Solver", "(Esc) to return", (20, 80));
}
