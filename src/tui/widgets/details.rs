use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::tui::layout::FocusedArea;

use super::styled_title;

pub fn render(frame: &mut Frame, area: Rect, focused: FocusedArea) {
    let color = if focused == FocusedArea::Details {
        Color::White
    } else {
        Color::DarkGray
    };

    let block = Block::default()
        .title(styled_title("Details", true))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(color));

    let widget = Paragraph::new("TEST CONTENT").block(block);
    frame.render_widget(widget, area);
}
