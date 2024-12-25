use std::collections::HashMap;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

use crate::core::helpers;

pub fn draw(frame: &mut Frame, area: Rect, selection: u8, input_buffer: &mut HashMap<u8, String>) {
    let mut header_name = "".to_string();
    let mut header_value = "".to_string();
    let widths = helpers::get_width_by_ratio(area.width, [4, 4, 1].to_vec());
    let fields: [Rect; 3] = Layout::horizontal([
        Constraint::Min(*widths.get(0).unwrap()),
        Constraint::Min(*widths.get(1).unwrap()),
        Constraint::Length(*widths.get(2).unwrap()),
    ])
    .areas(area);

    header_name.push_str(input_buffer.get(&0).unwrap_or(&"".to_string()));
    header_value.push_str(input_buffer.get(&1).unwrap_or(&"".to_string()));

    frame.render_widget(
        Paragraph::new(header_name).block(Block::bordered().border_style(if selection == 0 {
            Color::Blue
        } else {
            Color::Gray
        })),
        *fields.get(0).unwrap(),
    );
    frame.render_widget(
        Paragraph::new(header_value).block(Block::bordered()),
        *fields.get(1).unwrap(),
    );
    frame.render_widget(
        Paragraph::new("Add").block(Block::bordered()),
        *fields.get(2).unwrap(),
    );
}
