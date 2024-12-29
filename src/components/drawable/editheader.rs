use std::collections::HashMap;

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::core::helpers;
use crate::core::theme;

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
        //Paragraph::new(header_name).block(theme::set_border_color(selection == 0)),
        Paragraph::new(header_name).block(theme::set_input_block(selection == 0)),
        *fields.get(0).unwrap(),
    );
    frame.render_widget(
        Paragraph::new(header_value).block(theme::set_input_block(selection == 1)),
        *fields.get(1).unwrap(),
    );
    frame.render_widget(
        Paragraph::new("Add")
            .alignment(Alignment::Center)
            .block(theme::set_button_block(selection == 2))
            .style(theme::set_button_style(selection == 2)),
        *fields.get(2).unwrap(),
    );
}
