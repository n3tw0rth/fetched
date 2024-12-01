use ratatui::layout::Rect;
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::Frame;

use crate::core::{enums, helpers};

pub fn draw(frame: &mut Frame, area: Rect) {
    let ratio = 1.0 / 6.0;
    let rect = helpers::find_position(enums::ContainerPositions::Bottom, ratio, area);
    frame.render_widget(
        Paragraph::new("This area will be used to define header values")
            .block(Block::bordered().padding(Padding::new(0, 0, 0, 0))),
        rect,
    );
}
