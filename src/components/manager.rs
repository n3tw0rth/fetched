use std::error::Error;

use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use super::structs::App;
use super::widget_contents;
use crate::core::enums::{RequestWidgetTabs, ResponseWidgetTabs};
use strum::IntoEnumIterator;

pub fn match_request_widget_with_opened_tab<'a>(
    state: &mut App,
    frame: &mut Frame,
    scroll_items: Vec<Line>,
    area: Rect,
) -> Result<(), Box<dyn Error>> {
    let selected_tab = RequestWidgetTabs::iter().nth(state.selected_tab).unwrap();
    match selected_tab {
        RequestWidgetTabs::Body => widget_contents::body::render(),
        RequestWidgetTabs::Query => widget_contents::query::render(),
        RequestWidgetTabs::Header => {
            widget_contents::header::render(state, frame, scroll_items, area)
        }
        RequestWidgetTabs::Authentication => widget_contents::auth::render(),
    }
    .unwrap();

    Ok(())
}

pub fn match_response_widget_with_opened_tab<'a>(
    selected_tab: ResponseWidgetTabs,
) -> Result<Paragraph<'a>, Box<dyn Error>> {
    match selected_tab {
        ResponseWidgetTabs::ResponseBody => widget_contents::response_body::render(),
        ResponseWidgetTabs::ResponseHeader => widget_contents::response_header::render(),
    }
}
