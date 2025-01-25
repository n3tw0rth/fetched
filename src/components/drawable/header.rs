use ratatui::layout::Rect;
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, area: Rect) {
    let header_text = Paragraph::new("header text").block(Block::new().padding(Padding::left(1)));
    frame.render_widget(header_text, area);
}
