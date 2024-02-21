use ratatui::{
    Frame,
    text::Span,
    style::{Color, Style, Stylize},
    layout::Rect,
    widgets::{Paragraph, Block, Borders}
};

use super::util;

pub fn display(frame: &mut Frame, body_section: Rect, footer_section: Rect) {
    body_display(frame, body_section);
    footer_display(frame, footer_section);
}

fn body_display(frame: &mut Frame, body_section: Rect) {
    let body_section = util::centered_rect(60, 24, body_section);

    let confirmation_message = Paragraph::new(Span::styled(
        "Are you sure you want to quit",
        Style::default().fg(Color::Black)
    )).block(Block::default().borders(Borders::ALL).title("Quit").bg(Color::Gray)).fg(Color::Black);

    frame.render_widget(confirmation_message, body_section);
}

fn footer_display(frame: &mut Frame, footer_section: Rect) {
    util::footer_display(
        frame,
        footer_section,
        "Exiting",
        "(y) Yes / (n) No",
        (30, 70)
    );
}