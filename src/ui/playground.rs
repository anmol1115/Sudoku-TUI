use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Cell, Row, Table},
    Frame,
};

use super::util;
use crate::app;

pub fn display(frame: &mut Frame, body_section: Rect, footer_section: Rect, app: &mut app::App) {
    body_display(frame, body_section, app);
    footer_display(frame, footer_section);
}

fn body_display(frame: &mut Frame, body_section: Rect, app: &mut app::App) {
    let body_section = util::centered_rect(70, 60, body_section);

    let widths = vec![Constraint::Length(10); 9];
    let mut rows = Vec::new();

    let active_style = Style::default().bg(Color::Gray).fg(Color::Black);
    let invalid_style = Style::default().fg(Color::Red);
    let valid_style = Style::default().fg(Color::Green);
    let existing_style = Style::default().fg(Color::Yellow);

    for r in 0..9 {
        let mut column = Vec::new();
        for c in 0..9 {
            let mut entry = Cell::from(app.problem[r][c].to_string().clone());
            if app.invalid_entries.contains(&(r, c)) {
                entry = entry.clone().style(invalid_style);
            } else if app.valid_entries.contains(&(r, c)) {
                entry = entry.clone().style(valid_style);
            }

            if !app.playable_pos.contains(&(r, c)) {
                entry = entry.clone().style(existing_style);
            }

            if r == app.selected_row && c == app.selected_col {
                column.push(entry.style(active_style));
            } else {
                column.push(entry);
            }
        }
        rows.push(Row::new(column).height(2));
    }

    let grid = Table::new(rows, widths);

    frame.render_widget(grid, body_section);
}

fn footer_display(frame: &mut Frame, footer_section: Rect) {
    util::footer_display(
        frame,
        footer_section,
        "Playground",
        "(Arrows) to navigate / (0-9) for an entry / (Esc) to return / (v) to validate / (r) to reset / (s) to solve",
        (15, 85))
}
