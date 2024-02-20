use ratatui::{
    Frame,
    layout::{Rect, Layout, Direction, Constraint},
    widgets::{Paragraph, Block, Borders, ListItem, List},
    text::{Span, Line},
    style::{Style, Color, Stylize}
};

use crate::app;
use super::centered_rect;

pub fn display(frame: &mut Frame, body_section: Rect, footer_section: Rect, app: &mut app::App) {
    body_display(frame, body_section, app);
    footer_display(frame, footer_section);
}

fn body_display(frame: &mut Frame, body_section: Rect, app: &mut app::App) {
    let body_section = centered_rect(90, 60, body_section);
    let mut list_items = Vec::<ListItem>::new();

    for file in &app.all_files {
        list_items.push(ListItem::new(Line::from(Span::styled(
            file.clone(),
            Style::default()
        ))))   
    }

    let active_style = Style::default().bg(Color::White).fg(Color::Black);
    list_items[app.selected_file] = list_items[app.selected_file].clone().style(active_style);

    let list = List::new(list_items);
    frame.render_widget(list, body_section);
}

fn footer_display(frame: &mut Frame, footer_section: Rect) {
    let navigation_text = Paragraph::new(Span::styled(
        "Difficulty Select",
        Style::default().fg(Color::Black)
    )).block(Block::default().borders(Borders::ALL).bg(Color::Gray)).fg(Color::Black);

    let hint_text = Paragraph::new(Span::styled(
        "(Up/Down) to navigate / (Esc) to return / (Enter) to select",
        Style::default().fg(Color::Black)
    )).block(Block::default().borders(Borders::ALL).bg(Color::Gray)).fg(Color::Black);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70)
        ])
        .split(footer_section);

    frame.render_widget(navigation_text, footer_chunks[0]);
    frame.render_widget(hint_text, footer_chunks[1]);
}