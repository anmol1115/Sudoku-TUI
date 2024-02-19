use ratatui::{
    Frame,
    text::Span,
    widgets::{Paragraph, Block, Borders},
    style::{Color, Style},
    layout::{Direction, Constraint, Layout, Rect}
};

mod menu;
use crate::app;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let v_center = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100-percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100-percent_y) / 2)
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100-percent_x)/2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100-percent_x)/2)
        ])
        .split(v_center[1])[1]
}

pub fn ui(frame: &mut Frame, app: &mut app::App) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(9),
            Constraint::Length(3)
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Span::styled(
        "Sudoku in CLI",
        Style::default().fg(Color::Green)
    )).block(title_block);

    frame.render_widget(title, sections[0]);

    match app.current_screen {
        app::CurrentScreen::Menu => menu::display(frame, sections[1], sections[2], app),
        _ => ()
    };
}