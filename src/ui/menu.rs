use ratatui::{
    Frame,
    layout::{Rect, Layout, Direction, Constraint},
    widgets::{Paragraph, Block, Borders},
    style::{Color, Style}
};

use crate::app;
use super::util;

pub fn display(frame: &mut Frame, body_section: Rect, footer_section: Rect, app: &mut app::App) {
    body_display(frame, body_section, app);
    footer_display(frame, footer_section);
}

fn body_display(frame: &mut Frame, body_section: Rect, app: &mut app::App) {
    let body_section = util::centered_rect(90, 10, body_section);
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(body_section);

    let mut select_diff_block = Block::default().borders(Borders::ALL);
    let mut random_diff_block = Block::default().borders(Borders::ALL);

    let active_style = Style::default().bg(Color::White).fg(Color::Black);

    match app.menu_selection {
        app::MenuSelection::ChooseDifficulty => {select_diff_block = select_diff_block.style(active_style)},
        app::MenuSelection::RandomDifficulty => {random_diff_block = random_diff_block.style(active_style)},
    }

    let select_diff_text = Paragraph::new("Select Difficulty").block(select_diff_block);
    frame.render_widget(select_diff_text, body_chunks[0]);

    let random_diff_text = Paragraph::new("Random Difficulty").block(random_diff_block);
    frame.render_widget(random_diff_text, body_chunks[1])
}

fn footer_display(frame: &mut Frame, footer_section: Rect) {
    util::footer_display(
        frame,
        footer_section,
        "Menu",
        "(Tab) to switch / (Esc) to quit / (Enter) to select",
        (30, 70)
    );
}