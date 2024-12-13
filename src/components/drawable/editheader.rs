use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("some content here").block(Block::bordered()),
        area,
    );
}
