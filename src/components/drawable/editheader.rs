use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, area: Rect, input: &String) {
    frame.render_widget(
        Paragraph::new(input.to_string()).block(Block::bordered()),
        area,
    );
}
