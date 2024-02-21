use ratatui::{
    Frame,
    layout::Rect,
    widgets::{ListItem, List},
    text::{Span, Line},
    style::{Style, Color}
};

use crate::app;
use super::util;

pub fn display(frame: &mut Frame, body_section: Rect, footer_section: Rect, app: &mut app::App) {
    body_display(frame, body_section, app);
    footer_display(frame, footer_section);
}

fn body_display(frame: &mut Frame, body_section: Rect, app: &mut app::App) {
    let body_section = util::centered_rect(90, 60, body_section);
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
    util::footer_display(
        frame,
        footer_section,
        "Difficulty Select",
        "(Up/Down) to navigate / (Esc) to return / (Enter) to select",
        (30, 70)
    );
}