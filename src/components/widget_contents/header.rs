use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation};
use ratatui::Frame;

use crate::components::structs::App;

pub fn render<'a>(
    state: &mut App,
    frame: &mut Frame,
    mut scroll_items: Vec<Line>,
    area: Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    state.vertical_scroll_state = state
        .vertical_scroll_state
        .content_length(state.request_data.headers.len());

    // render header values onto terminal
    for header in state.request_data.headers.clone() {
        scroll_items.push(Line::from(format!("{}\t\t{}", header.0, header.1)));
    }

    let paragraph = Paragraph::new(scroll_items).scroll((0, 0));
    frame.render_widget(paragraph, area);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .end_symbol(None)
            .begin_symbol(None),
        area,
        &mut state.vertical_scroll_state,
    );

    Ok(())
}
