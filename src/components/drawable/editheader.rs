use std::collections::HashMap;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::theme;

pub fn draw(frame: &mut Frame, area: Rect, selection: u8, input_buffer: &mut HashMap<u8, String>) {
    let mut header_name = "".to_string();
    let mut header_value = "".to_string();
    let areas: [Rect; 3] = Layout::horizontal([
        Constraint::Ratio(3, 7),
        Constraint::Ratio(3, 7),
        Constraint::Ratio(1, 7),
    ])
    .areas(area);

    header_name.push_str(input_buffer.get(&0).unwrap_or(&"".to_string()));
    header_value.push_str(input_buffer.get(&1).unwrap_or(&"".to_string()));

    frame.render_widget(
        Paragraph::new(header_name).block(theme::set_input_block(selection == 0)),
        *areas.get(0).unwrap(),
    );
    frame.render_widget(
        Paragraph::new(header_value).block(theme::set_input_block(selection == 1)),
        *areas.get(1).unwrap(),
    );
    frame.render_widget(
        Paragraph::new("ADD")
            .centered()
            .block(theme::set_button_block(selection == 2))
            .style(theme::set_button_style(selection == 2)),
        *areas.get(2).unwrap(),
    );
}
