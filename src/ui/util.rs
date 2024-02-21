use ratatui::{
    Frame,
    text::Span,
    layout::{Rect, Layout, Direction, Constraint},
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color, Stylize}
};

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

pub fn footer_display(frame: &mut Frame, footer_section: Rect, screen_mode: &str, screen_hints: &str, ratio: (u16, u16)) {
    let navigation_text = Paragraph::new(Span::styled(
        screen_mode,
        Style::default().fg(Color::Black)
    )).block(Block::default().borders(Borders::ALL).bg(Color::Gray)).fg(Color::Black);

    let hint_text = Paragraph::new(Span::styled(
        screen_hints,
        Style::default().fg(Color::Black)
    )).block(Block::default().borders(Borders::ALL).bg(Color::Gray)).fg(Color::Black);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(ratio.0),
            Constraint::Percentage(ratio.1)
        ])
        .split(footer_section);

    frame.render_widget(navigation_text, footer_chunks[0]);
    frame.render_widget(hint_text, footer_chunks[1]);
}